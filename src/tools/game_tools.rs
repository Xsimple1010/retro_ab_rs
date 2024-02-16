use std::{
    ffi::CString,
    fs::File,
    io::Read,
    os::raw::c_void,
    path::{Path, PathBuf},
    ptr::null,
    sync::Arc,
};

use crate::{
    binding::binding_libretro::{retro_game_info, LibretroRaw},
    core::RetroContext,
    erro_handle::{ErroHandle, RetroLogLevel},
};

use super::ffi_tools::make_c_string;

fn get_full_path(path: &str) -> Result<PathBuf, ErroHandle> {
    match PathBuf::from(path).canonicalize() {
        Ok(full_path) => Ok(full_path),
        Err(e) => Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: e.to_string(),
        }),
    }
}

fn valid_rom_extension(ctx: &Arc<RetroContext>, path: &Path) -> Result<(), ErroHandle> {
    let valid_extensions = ctx.core.system.info.valid_extensions.lock().unwrap();
    let path_str = path.extension().unwrap().to_str().unwrap();

    if !valid_extensions.contains(path_str) {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "Extensão da rom invalida: valores esperados -> ".to_string()
                + &valid_extensions.to_string()
                + "; valor recebido -> "
                + path_str,
        });
    };

    Ok(())
}

pub fn create_game_info(
    ctx: &Arc<RetroContext>,
    raw: &LibretroRaw,
    path: &str,
) -> Result<bool, ErroHandle> {
    let f_path = get_full_path(path)?;

    valid_rom_extension(ctx, &f_path)?;

    let mut buf = Vec::new();
    let meta = CString::new("").unwrap();
    let path = make_c_string(f_path.to_str().unwrap())?;
    let mut size = 0;

    let need_full_path = *ctx.core.system.info.need_fullpath.lock().unwrap();

    if !need_full_path {
        let mut file = File::open(f_path).unwrap();

        let len = file.metadata().unwrap().len() as usize;

        buf = Vec::with_capacity(len);
        size = len;

        file.read_to_end(&mut buf).unwrap();
    }

    let game_info = retro_game_info {
        data: if buf.is_empty() {
            null()
        } else {
            buf.as_ptr() as *const c_void
        },
        meta: meta.as_ptr(),
        path: path.as_ptr(),
        size,
    };

    let state = unsafe { raw.retro_load_game(&game_info) };

    Ok(state)
}
