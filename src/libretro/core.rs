use super::{
    binding_libretro::{
        retro_audio_sample_batch_t, retro_audio_sample_t, retro_input_poll_t, retro_input_state_t,
        retro_video_refresh_t, LibretroRaw,
    },
    environment,
};

pub struct Callbacks {
    pub video_refresh_callback: retro_video_refresh_t,

    pub audio_sample_callback: retro_audio_sample_t,

    pub audio_sample_batch_callback: retro_audio_sample_batch_t,

    pub input_poll_callback: retro_input_poll_t,

    pub input_state_callback: retro_input_state_t,
}

pub struct CoreWrapper {
    raw: LibretroRaw,

    pub can_dupe: bool,
    pub had_frame: bool,
    pub last_width: u32,
    pub last_height: u32,
    pub last_pitch: usize,

    pub supports_bitmasks: bool,

    pub frame_delta: Option<i64>,
}

impl CoreWrapper {
    pub fn rum(&self) {
        unsafe {
            self.raw.retro_run();
        }
    }

    pub fn de_init(&self) {
        unsafe {
            self.raw.retro_deinit();
        }
    }

    pub fn version(&self) -> u32 {
        unsafe { self.raw.retro_api_version() }
    }

    pub fn init(&self) {
        unsafe { self.raw.retro_init() }
    }
}

pub fn load(path: &String, callbacks: &Callbacks) -> Result<CoreWrapper, ::libloading::Error> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                libretro_raw.retro_set_audio_sample(callbacks.audio_sample_callback);
                libretro_raw.retro_set_audio_sample_batch(callbacks.audio_sample_batch_callback);
                libretro_raw.retro_set_video_refresh(callbacks.video_refresh_callback);
                libretro_raw.retro_set_input_poll(callbacks.input_poll_callback);
                libretro_raw.retro_set_input_state(callbacks.input_state_callback);
                libretro_raw.retro_set_environment(Some(environment::core_environment));

                let core_wrapper = CoreWrapper {
                    raw: libretro_raw,
                    can_dupe: false,
                    frame_delta: Some(0),
                    had_frame: false,
                    last_height: 0,
                    last_width: 0,
                    last_pitch: 0,
                    supports_bitmasks: false,
                };

                Ok(core_wrapper)
            }
            Err(e) => Err(e),
        }
    }
}
