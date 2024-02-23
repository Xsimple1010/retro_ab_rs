use crate::{
    av_info::update_av_info,
    binding::binding_libretro::LibretroRaw,
    environment,
    erro_handle::{ErroHandle, RetroLogLevel},
    managers,
    paths::Paths,
    retro_context,
    system::System,
    tools,
};
use std::sync::{Arc, Mutex};

pub use crate::av_info::{AvInfo, Geometry, Timing, Video};
pub use crate::binding::binding_libretro::retro_language;
pub use crate::binding::binding_libretro::retro_pixel_format;
pub use crate::environment::RetroEnvCallbacks;
pub use crate::retro_context::RetroContext;
pub use managers::option_manager::update_opt;
pub use retro_context::get_num_context;

pub struct CoreWrapper {
    pub initialized: Mutex<bool>,
    pub game_loaded: Mutex<bool>,
    pub supports_bitmasks: Mutex<bool>,
    pub support_no_game: Mutex<bool>,
    pub language: Mutex<retro_language>,
    pub av_info: Arc<AvInfo>,
    pub system: System,
    raw: Arc<LibretroRaw>,
}

impl CoreWrapper {
    pub fn new(raw: LibretroRaw) -> CoreWrapper {
        CoreWrapper {
            raw: Arc::new(raw),
            initialized: Mutex::new(false),
            game_loaded: Mutex::new(false),
            support_no_game: Mutex::new(false),
            //TODO:precisa modificado de acordo com o idioma selecionado no sistema operacional!
            language: Mutex::new(retro_language::RETRO_LANGUAGE_PORTUGUESE_BRAZIL),
            supports_bitmasks: Mutex::new(false),
            system: System::default(),
            av_info: Arc::new(AvInfo::default()),
        }
    }
}

pub fn run(ctx: &Arc<RetroContext>) -> Result<(), ErroHandle> {
    if !*ctx.core.initialized.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "O núcleo nao foi inicializado".to_string(),
        });
    }

    if !*ctx.core.game_loaded.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_WARN,
            message: "Nao ha nenhuma rum carregada no momento".to_string(),
        });
    }

    unsafe {
        ctx.core.raw.retro_run();
    }

    Ok(())
}

pub fn de_init(ctx: Arc<RetroContext>) -> Result<(), ErroHandle> {
    if !*ctx.core.initialized.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_WARN,
            message: "Nao e possível descarrega o núcleo, pois o mesmo ainda nao foi inicializado!"
                .to_string(),
        });
    }

    //se uma rom estive carrega ela deve ser descarregada primeiro
    unload_game(&ctx)?;

    unsafe {
        ctx.core.raw.retro_deinit();
        *ctx.core.initialized.lock().unwrap() = false;

        environment::delete_local_ctx();
        retro_context::delete(ctx);
    }

    Ok(())
}

pub fn version(ctx: &Arc<RetroContext>) -> u32 {
    unsafe { ctx.core.raw.retro_api_version() }
}

pub fn init(ctx: &Arc<RetroContext>) -> Result<(), ErroHandle> {
    if *ctx.core.game_loaded.lock().unwrap() || *ctx.core.initialized.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_WARN,
            message: "Para inicializar um novo núcleo e necessário descarrega o núcleo atual"
                .to_string(),
        });
    }

    unsafe {
        *ctx.core.initialized.lock().unwrap() = true;
        ctx.core.raw.retro_init();

        Ok(())
    }
}

pub fn load_game(ctx: &Arc<RetroContext>, path: &str) -> Result<bool, ErroHandle> {
    if *ctx.core.game_loaded.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_WARN,
            message: "Ja existe uma rom carregada no momento".to_string(),
        });
    }

    if !*ctx.core.initialized.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "Para carregar uma rom o núcleo deve esta inicializado".to_string(),
        });
    }

    match tools::game_tools::create_game_info(ctx, &ctx.core.raw, path) {
        Ok(state) => {
            *ctx.core.game_loaded.lock().unwrap() = state;
            update_av_info(ctx, &ctx.core.raw);
            Ok(state)
        }
        Err(e) => Err(e),
    }
}

pub fn unload_game(ctx: &Arc<RetroContext>) -> Result<(), ErroHandle> {
    if !*ctx.core.game_loaded.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_WARN,
            message: "A rom ja foi descarregada anteriormente".to_string(),
        });
    }

    if !*ctx.core.initialized.lock().unwrap() {
        return Err(ErroHandle {
            level: RetroLogLevel::RETRO_LOG_ERROR,
            message: "Para descarregar uma rom o núcleo deve esta inicializado".to_string(),
        });
    }

    unsafe {
        ctx.core.raw.retro_unload_game();
    }
    *ctx.core.game_loaded.lock().unwrap() = false;

    Ok(())
}

pub fn load(
    path: &str,
    paths: Paths,
    callbacks: RetroEnvCallbacks,
) -> Result<Arc<RetroContext>, String> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                let context = retro_context::create(libretro_raw, paths, callbacks);

                environment::configure(Arc::clone(&context));

                context
                    .core
                    .raw
                    .retro_set_environment(Some(environment::core_environment));

                context
                    .core
                    .raw
                    .retro_set_audio_sample(Some(environment::audio_sample_callback));

                context
                    .core
                    .raw
                    .retro_set_audio_sample_batch(Some(environment::audio_sample_batch_callback));

                context
                    .core
                    .raw
                    .retro_set_video_refresh(Some(environment::video_refresh_callback));

                context
                    .core
                    .raw
                    .retro_set_input_poll(Some(environment::input_poll_callback));

                context
                    .core
                    .raw
                    .retro_set_input_state(Some(environment::input_state_callback));

                Ok(Arc::clone(&context))
            }
            Err(_) => Err(String::from("Erro ao carregar o núcleo: ")),
        }
    }
}

//TODO: adicionar testes aqui
