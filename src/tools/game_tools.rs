use std::{ffi::CString, path::PathBuf};

use crate::{
    erro_handle::{ErroHandle, Level},
    libretro::binding_libretro::retro_game_info,
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

pub fn create_game_info(path: &str) -> Result<retro_game_info, ErroHandle> {
    let meta = CString::new("").unwrap();
    let path = get_full_path(path)?;

    let game_info = retro_game_info {
        data: std::ptr::null(),
        meta: meta.as_ptr(),
        path: path.as_ptr(),
        size: 0,
    };

    Ok(game_info)
}
