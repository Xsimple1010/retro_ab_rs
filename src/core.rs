use std::sync::{Arc, Mutex};

use super::environment;
use crate::{
    binding_libretro::{retro_language, retro_pixel_format, LibretroRaw},
    game_tools,
    retro_context::{self, get_num_context},
};

pub use crate::retro_context::RetroContext;

pub struct SysInfo {
    pub library_name: Mutex<String>,
    pub library_version: Mutex<String>,
    pub valid_extensions: Mutex<String>,
    pub need_fullpath: Mutex<bool>,
    pub block_extract: Mutex<bool>,
}

impl SysInfo {
    fn new() -> SysInfo {
        SysInfo {
            library_name: Mutex::new("".to_owned()),
            library_version: Mutex::new("".to_owned()),
            valid_extensions: Mutex::new("".to_owned()),
            block_extract: Mutex::new(false),
            need_fullpath: Mutex::new(false),
        }
    }
}
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
    pub pixel_format: Mutex<retro_pixel_format>,
    pub frame_delta: Option<i64>,
}

pub struct CoreWrapper {
    pub initialized: Mutex<bool>,
    pub video: Video,
    pub supports_bitmasks: Mutex<bool>,
    pub support_no_game: Mutex<bool>,
    pub use_subsystem: Mutex<bool>,
    pub language: Mutex<retro_language>,
    pub sys_info: SysInfo,
    raw: Arc<LibretroRaw>,
}

impl CoreWrapper {
    pub fn new(raw: LibretroRaw) -> CoreWrapper {
        CoreWrapper {
            raw: Arc::new(raw),
            initialized: Mutex::new(false),
            support_no_game: Mutex::new(false),
            use_subsystem: Mutex::new(false),
            language: Mutex::new(retro_language::RETRO_LANGUAGE_PORTUGUESE_BRAZIL),
            supports_bitmasks: Mutex::new(false),
            sys_info: SysInfo::new(),
            video: Video {
                can_dupe: false,
                frame_delta: Some(0),
                had_frame: false,
                last_height: 0,
                last_width: 0,
                last_pitch: 0,
                pixel_format: Mutex::new(retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN),
            },
        }
    }
}

pub fn run(ctx: &Arc<RetroContext>) {
    unsafe {
        ctx.core.raw.retro_run();
    }
}

pub fn de_init(ctx: Arc<RetroContext>) {
    unsafe {
        ctx.core.raw.retro_deinit();
    }
}

pub fn version(ctx: &Arc<RetroContext>) -> u32 {
    unsafe { ctx.core.raw.retro_api_version() }
}

pub fn init(ctx: &Arc<RetroContext>) {
    unsafe {
        ctx.core.raw.retro_init();
    }
}

pub fn load_game(ctx: &Arc<RetroContext>, path: &str) {
    game_tools::load(&ctx.core.raw, path);
}

pub fn load(path: &String, callbacks: CoreCallbacks) -> Result<Arc<RetroContext>, String> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                let context = Some(retro_context::create(libretro_raw, callbacks));

                match &context {
                    Some(ctx) => {
                        environment::configure(Arc::clone(ctx));

                        ctx.core
                            .raw
                            .retro_set_environment(Some(environment::core_environment));
                        ctx.core
                            .raw
                            .retro_set_audio_sample(Some(environment::audio_sample_callback));
                        ctx.core.raw.retro_set_audio_sample_batch(Some(
                            environment::audio_sample_batch_callback,
                        ));
                        ctx.core
                            .raw
                            .retro_set_video_refresh(Some(environment::video_refresh_callback));
                        ctx.core
                            .raw
                            .retro_set_input_poll(Some(environment::input_poll_callback));
                        ctx.core
                            .raw
                            .retro_set_input_state(Some(environment::input_state_callback));

                        println!("ctxS len {:?}", get_num_context());

                        Ok(Arc::clone(ctx))
                    }
                    None => Err(String::from("value")),
                }
            }
            Err(_) => Err(String::from("Erro ao carregar o núcleo: ")),
        }
    }
}
