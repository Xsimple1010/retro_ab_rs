use std::{ffi::c_void, rc::Rc};

use crate::libretro::binding_libretro::{retro_system_av_info, retro_system_info};

use super::{
    binding_libretro::{retro_game_info, LibretroRaw},
    environment, game_tools,
};

pub struct CoreCallbacks {
    pub video_refresh_callback:
        fn(data: *const ::std::os::raw::c_void, width: i32, height: i32, pitch: usize),

    pub audio_sample_callback: fn(left: i16, right: i16),

    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,

    pub input_poll_callback: fn(),

    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
}

pub struct CoreWrapper {
    pub can_dupe: bool,
    pub had_frame: bool,
    pub last_width: u32,
    pub last_height: u32,
    pub last_pitch: usize,

    pub supports_bitmasks: bool,

    pub frame_delta: Option<i64>,
}

impl CoreWrapper {
    pub fn run(&self) {
        unsafe {
            match &RAW {
                Some(raw) => raw.retro_run(),
                None => {}
            }
        }
    }

    pub fn de_init(&self) {
        unsafe {
            match &RAW {
                Some(raw) => raw.retro_deinit(),
                None => {}
            }
        }
    }

    pub fn version(&self) -> u32 {
        unsafe {
            match &RAW {
                Some(raw) => raw.retro_api_version(),
                None => 0,
            }
        }
    }

    pub fn init(&self) {
        unsafe {
            match &RAW {
                Some(raw) => raw.retro_init(),
                None => {}
            }
        }
    }

    pub fn load_game(&self, path: String) {
        unsafe {
            match &RAW {
                Some(raw) => game_tools::load(raw, &path),
                None => {}
            }
        }
    }
}

static mut RAW: Option<LibretroRaw> = None;

pub fn load(path: &String, callbacks: CoreCallbacks) -> Result<Rc<CoreWrapper>, String> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                let core_wrapper = CoreWrapper {
                    can_dupe: false,
                    frame_delta: Some(0),
                    had_frame: false,
                    last_height: 0,
                    last_width: 0,
                    last_pitch: 0,
                    supports_bitmasks: false,
                };

                //configure all needed callbacks
                RAW = Some(libretro_raw);
                let core_wrapper = environment::configure(core_wrapper, callbacks);

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

                match core_wrapper {
                    Ok(core) => Ok(core),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(String::from("Erro ao carregar o núcleo: ")),
        }
    }
}
