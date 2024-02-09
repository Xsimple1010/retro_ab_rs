use std::{ffi::CString, path::PathBuf};

use crate::{
    binding_libretro::{retro_game_info, LibretroRaw},
    erro_handle::{ErroHandle, Level},
};

fn get_full_path(path: &str) -> Result<CString, ErroHandle> {
    match PathBuf::from(path).canonicalize() {
        Ok(full_path) => {
            let path = full_path.to_str().to_owned().unwrap().to_string();

            match CString::new(path) {
                Ok(c_string) => Ok(c_string),
                _ => Err(ErroHandle {
                    level: Level::Erro,
                    message: "Nao foi possível cria uma c_string".to_string(),
                }),
            }
        }
        Err(e) => Err(ErroHandle {
            level: Level::Erro,
            message: e.to_string(),
        }),
    }
}

pub fn load(raw: &LibretroRaw, path: &str) -> Result<bool, ErroHandle> {
    let meta = CString::new("").unwrap();
    let path = get_full_path(path)?;

    let _game_info = retro_game_info {
        data: std::ptr::null(),
        meta: meta.as_ptr(),
        path: path.as_ptr(),
        size: 0,
    };

    unsafe {
        let loaded = raw.retro_load_game(&_game_info);

        Ok(loaded)
    }
}
