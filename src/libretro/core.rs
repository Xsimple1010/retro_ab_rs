use std::ffi::c_void;

use crate::libretro::binding_libretro::{retro_system_av_info, retro_system_info};

use super::{
    binding_libretro::{retro_game_info, LibretroRaw},
    environment,
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
                Some(raw) => {
                    const DATA: *const c_void = std::ptr::null();
                    let sys_av_info: *mut retro_system_info = &mut retro_system_info {
                        block_extract: false,
                        need_fullpath: false,
                        library_name: "".as_ptr() as *const i8,
                        library_version: "".as_ptr() as *const i8,
                        valid_extensions: "".as_ptr() as *const i8,
                    };

                    let game_info: *mut retro_game_info = &mut retro_game_info {
                        data: DATA,
                        meta: "".as_ptr() as *const i8,
                        path: path.as_ptr() as *const i8,
                        size: 0,
                    };

                    println!("{:?}", path);

                    raw.retro_get_system_info(sys_av_info);

                    if sys_av_info.read().need_fullpath {
                        println!("fullpath sim");
                    }

                    // let av_info = raw.retro_load_game(game_info);
                }
                None => {}
            }
        }
    }
}

static mut RAW: Option<LibretroRaw> = None;

pub fn load(path: &String, callbacks: CoreCallbacks) -> Result<&'static CoreWrapper, String> {
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
