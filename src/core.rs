pub use crate::av_info::{AvInfo, Geometry, Timing, Video};
pub use crate::binding::binding_libretro::retro_language;
pub use crate::binding::binding_libretro::retro_pixel_format;
pub use crate::environment::RetroEnvCallbacks;
use crate::erro_handle::{ErroHandle, RetroLogLevel};
use crate::graphic_api::GraphicApi;
use crate::{
    binding::binding_libretro::LibretroRaw, environment, managers::option_manager::OptionManager,
    paths::Paths, system::System, tools,
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type CoreWrapperIns = Arc<CoreWrapper>;

pub struct CoreWrapper {
    /// # Retro_context_associated
    ///
    /// Adicionei isso com o proposito de chamar futuras callbacks que serão adicionadas
    /// [RetroContext] dentro das callbacks fornecidas por [environment],
    pub retro_ctx_associated: Uuid,
    pub initialized: Mutex<bool>,
    pub game_loaded: Mutex<bool>,
    pub supports_bitmasks: Mutex<bool>,
    pub support_no_game: Mutex<bool>,
    pub language: Mutex<retro_language>,
    pub av_info: Arc<AvInfo>,
    pub system: System,
    pub paths: Paths,
    pub options: OptionManager,
    pub raw: Arc<LibretroRaw>,
    pub callbacks: RetroEnvCallbacks,
}

impl Drop for CoreWrapper {
    fn drop(&mut self) {
        let _ = self.de_init();
    }
}

impl CoreWrapper {
    pub fn new(
        retro_ctx_associated: Uuid,
        core_path: &str,
        paths: Paths,
        callbacks: RetroEnvCallbacks,
        graphic_api: GraphicApi,
    ) -> Result<CoreWrapperIns, ErroHandle> {
        let raw = unsafe { LibretroRaw::new(core_path).unwrap() };

        let system = System::new(&raw);

        let options =
            OptionManager::new(&paths.opt, system.info.library_name.lock().unwrap().clone());

        let core = Arc::new(CoreWrapper {
            raw: Arc::new(raw),
            initialized: Mutex::new(false),
            game_loaded: Mutex::new(false),
            support_no_game: Mutex::new(false),
            av_info: Arc::new(AvInfo::new(graphic_api)),
            supports_bitmasks: Mutex::new(false),
            system,
            paths,
            options,
            callbacks,
            retro_ctx_associated,
            //TODO:precisa modificado de acordo com o idioma selecionado no sistema operacional!
            language: Mutex::new(retro_language::RETRO_LANGUAGE_PORTUGUESE_BRAZIL),
        });

        environment::configure(Arc::clone(&core));

        unsafe {
            core.raw
                .retro_set_environment(Some(environment::core_environment));

            core.raw
                .retro_set_audio_sample(Some(environment::audio_sample_callback));

            core.raw
                .retro_set_audio_sample_batch(Some(environment::audio_sample_batch_callback));

            core.raw
                .retro_set_video_refresh(Some(environment::video_refresh_callback));

            core.raw
                .retro_set_input_poll(Some(environment::input_poll_callback));

            core.raw
                .retro_set_input_state(Some(environment::input_state_callback));
        }

        Ok(core)
    }

    pub fn init(&self) -> Result<(), ErroHandle> {
        if *self.game_loaded.lock().unwrap() || *self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "Para inicializar um novo núcleo e necessário descarrega o núcleo atual"
                    .to_string(),
            });
        }

        unsafe {
            *self.initialized.lock().unwrap() = true;
            self.raw.retro_init();

            Ok(())
        }
    }

    pub fn load_game(&self, path: &str) -> Result<bool, ErroHandle> {
        if *self.game_loaded.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "Ja existe uma rom carregada no momento".to_string(),
            });
        }

        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_ERROR,
                message: "Para carregar uma rom o núcleo deve esta inicializado".to_string(),
            });
        }

        match tools::game_tools::create_game_info(self, path) {
            Ok(state) => {
                *self.game_loaded.lock().unwrap() = state;
                self.av_info.update_av_info(&self.raw);
                Ok(state)
            }
            Err(e) => Err(e),
        }
    }

    pub fn reset(&self) -> Result<(), ErroHandle> {
        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_ERROR,
                message: "O núcleo nao foi inicializado".to_string(),
            });
        }

        if !*self.game_loaded.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "Nao ha nenhuma rum carregada no momento".to_string(),
            });
        }

        unsafe {
            self.raw.retro_reset();
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), ErroHandle> {
        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_ERROR,
                message: "O núcleo nao foi inicializado".to_string(),
            });
        }

        if !*self.game_loaded.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "Nao ha nenhuma rum carregada no momento".to_string(),
            });
        }

        unsafe { self.raw.retro_run() }

        Ok(())
    }

    pub fn de_init(&self) -> Result<(), ErroHandle> {
        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message:
                    "Nao e possível descarrega o núcleo, pois o mesmo ainda nao foi inicializado!"
                        .to_string(),
            });
        }

        //Se uma *rom* estive carrega ela deve ser descarregada primeiro
        match self.unload_game() {
            Ok(..) => {}
            Err(e) => match &e.level {
                RetroLogLevel::RETRO_LOG_WARN => {}
                _ => {
                    unsafe {
                        self.raw.retro_deinit();
                    }
                    *self.initialized.lock().unwrap() = false;
                    environment::delete_local_core_ctx();

                    return Err(e);
                }
            },
        }

        unsafe {
            self.raw.retro_deinit();
        }
        *self.initialized.lock().unwrap() = false;
        environment::delete_local_core_ctx();

        Ok(())
    }

    pub fn connect_controller(&self, port: u32, controller: u32) -> Result<(), ErroHandle> {
        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "Nao é possível conectar um controle pois nenhum núcleo foi inicializado"
                    .to_string(),
            });
        }

        unsafe {
            self.raw.retro_set_controller_port_device(port, controller);
        }

        Ok(())
    }

    pub fn unload_game(&self) -> Result<(), ErroHandle> {
        if !*self.game_loaded.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_WARN,
                message: "A rom ja foi descarregada anteriormente".to_string(),
            });
        }

        if !*self.initialized.lock().unwrap() {
            return Err(ErroHandle {
                level: RetroLogLevel::RETRO_LOG_ERROR,
                message: "Para descarregar uma rom o núcleo deve esta inicializado".to_string(),
            });
        }

        unsafe {
            self.raw.retro_unload_game();
        }
        *self.game_loaded.lock().unwrap() = false;

        Ok(())
    }
}

#[cfg(test)]
mod core {}
