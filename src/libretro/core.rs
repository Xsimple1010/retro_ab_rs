use super::{
    binding_libretro::{retro_language, retro_pixel_format, LibretroRaw},
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
    pub video: Video,
    pub supports_bitmasks: bool,
    pub support_no_game: bool,
    pub use_subsystem: bool,
    pub language: retro_language,
}

static mut RAW: Option<LibretroRaw> = None;

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
            Some(raw) => raw.retro_deinit(),
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
            Some(raw) => raw.retro_init(),
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

pub fn load(
    path: &String,
    callbacks: CoreCallbacks,
) -> Result<&'static environment::Context, String> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => {
                let core_wrapper = CoreWrapper {
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
            Err(_) => Err(String::from("Erro ao carregar o núcleo: ")),
        }
    }
}
