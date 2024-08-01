use std::fs;
use std::ops::Not;
use std::path::Path;
use crate::erro_handle::ErroHandle;
use crate::erro_handle::RetroLogLevel::RETRO_LOG_ERROR;

#[derive(Clone, Debug)]
pub struct Paths {
    pub system: String,
    pub save: String,
    pub opt: String,
}


impl Paths {
    pub fn new(system: String, save: String, opt: String) -> Result<Self, ErroHandle> {
        if Path::new(&system).exists().not() {
            if fs::create_dir(&system).is_err() {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Não foi possível criar a pasta system".to_owned(),
                });
            }
        }

        if Path::new(&save).exists().not() {
            if fs::create_dir(&save).is_err() {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Não foi possível criar a pasta save".to_owned(),
                });
            }
        }

        if Path::new(&opt).exists().not() {
            if fs::create_dir(&opt).is_err() {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Não foi possível criar a pasta opt".to_owned(),
                });
            }
        }


        Ok(Paths {
            system,
            opt,
            save,
        })
    }
}