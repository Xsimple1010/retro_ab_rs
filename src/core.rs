use std::cell::RefCell;

use crate::{
    binding_libretro::{retro_language, retro_pixel_format, LibretroRaw},
    game_tools, option_manager,
};

use super::environment;

pub struct CoreCallbacks {
    pub video_refresh_callback:
        fn(data: *const ::std::os::raw::c_void, width: i32, height: i32, pitch: usize),

    pub audio_sample_callback: fn(left: i16, right: i16),

    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,

    pub input_poll_callback: fn(),

    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
}

pub struct Video {
    pub can_dupe: bool,
    pub had_frame: bool,
    pub last_width: u32,
    pub last_height: u32,
    pub last_pitch: usize,
    pub pixel_format: retro_pixel_format,
    pub frame_delta: Option<i64>,
}

pub struct CoreWrapper {
    pub initialized: bool,
    pub video: Video,
    pub supports_bitmasks: bool,
    pub support_no_game: bool,
    pub use_subsystem: bool,
    pub language: retro_language,
}

impl Default for CoreWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreWrapper {
    pub fn new() -> CoreWrapper {
        CoreWrapper {
            initialized: false,
            support_no_game: false,
            use_subsystem: false,
            language: retro_language::RETRO_LANGUAGE_PORTUGUESE_BRAZIL,
            supports_bitmasks: false,
            video: Video {
                can_dupe: false,
                frame_delta: Some(0),
                had_frame: false,
                last_height: 0,
                last_width: 0,
                last_pitch: 0,
                pixel_format: retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN,
            },
        }
    }
}

pub struct Context {
    pub core: RefCell<CoreWrapper>,
    pub callbacks: RefCell<CoreCallbacks>,
    pub options: RefCell<option_manager::OptionManager>,
}

static mut RAW: Option<LibretroRaw> = None;
static mut CONTEXT: Option<&'static Context> = None;

pub fn run() {
    unsafe {
        match &RAW {
            Some(raw) => raw.retro_run(),
            None => {}
        }
    }
}

pub fn de_init() {
    unsafe {
        match &RAW {
            Some(raw) => {
                if let Some(ctx) = CONTEXT {
                    raw.retro_deinit();
                    ctx.core.borrow_mut().initialized = false;
                }
            }
            None => {}
        }
    }
}

pub fn version() -> u32 {
    unsafe {
        match &RAW {
            Some(raw) => raw.retro_api_version(),
            None => 0,
        }
    }
}

pub fn init() {
    unsafe {
        match &RAW {
            Some(raw) => {
                if let Some(ctx) = CONTEXT {
                    raw.retro_init();
                    ctx.core.borrow_mut().initialized = true;
                }
            }
            None => {}
        }
    }
}

pub fn load_game(path: String) {
    unsafe {
        match &RAW {
            Some(raw) => game_tools::load(raw, &path),
            None => {}
        }
    }
}

pub fn load(path: &String, callbacks: CoreCallbacks) -> Result<&'static Context, String> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                let core_wrapper = CoreWrapper::new();

                //configure all needed callbacks
                RAW = Some(libretro_raw);
                let context = environment::configure(core_wrapper, callbacks);

                match &RAW {
                    Some(raw) => {
                        raw.retro_set_environment(Some(environment::core_environment));
                        raw.retro_set_audio_sample(Some(environment::audio_sample_callback));
                        raw.retro_set_audio_sample_batch(Some(
                            environment::audio_sample_batch_callback,
                        ));
                        raw.retro_set_video_refresh(Some(environment::video_refresh_callback));
                        raw.retro_set_input_poll(Some(environment::input_poll_callback));
                        raw.retro_set_input_state(Some(environment::input_state_callback));
                    }
                    None => {}
                }

                match context {
                    Ok(ctx) => {
                        CONTEXT = Some(ctx);

                        Ok(ctx)
                    }
                    Err(e) => Err(e),
                }
            }
            Err(_) => Err(String::from("Erro ao carregar o núcleo: ")),
        }
    }
}
