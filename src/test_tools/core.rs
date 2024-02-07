use super::constants::CORE_TEST_RELATIVE_PATH;
use crate::binding_libretro::LibretroRaw;
use crate::environment::RetroEnvCallbacks;

fn audio_sample_callback(_left: i16, _right: i16) {}

fn audio_sample_batch_callback(_data: *const i16, _frames: usize) -> usize {
    println!("{_frames}");
    0
}

fn input_poll_callback() {}

fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    println!("{_port} {_device}");
    0
}

fn video_refresh_callback(
    _data: *const ::std::os::raw::c_void,
    _width: i32,
    _height: i32,
    _pitch: usize,
) {
}

pub fn get_callbacks() -> RetroEnvCallbacks {
    RetroEnvCallbacks {
        audio_sample_batch_callback,
        audio_sample_callback,
        input_poll_callback,
        input_state_callback,
        video_refresh_callback,
    }
}

pub fn get_raw() -> Result<LibretroRaw, libloading::Error> {
    unsafe { LibretroRaw::new(CORE_TEST_RELATIVE_PATH) }
}
