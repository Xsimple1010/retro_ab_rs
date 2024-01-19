extern crate sdl2;
mod args_manager;
mod libretro;

use ::std::os::raw;

unsafe extern "C" fn audio_sample_callback(_left: i16, _right: i16) {}

unsafe extern "C" fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    frames
}

unsafe extern "C" fn input_poll_callback() {}

unsafe extern "C" fn input_state_callback(
    _port: raw::c_uint,
    _device: raw::c_uint,
    _index: raw::c_uint,
    _id: raw::c_uint,
) -> i16 {
    0
}

unsafe extern "C" fn video_refresh_callback(
    _data: *const raw::c_void,
    _width: raw::c_uint,
    _height: raw::c_uint,
    _pitch: usize,
) {
}

fn main() {
    let values = args_manager::get_values();

    if !values.is_empty() {
        let callbacks = libretro::core::Callbacks {
            audio_sample_callback: Some(audio_sample_callback),
            audio_sample_batch_callback: Some(audio_sample_batch_callback),
            input_poll_callback: Some(input_poll_callback),
            input_state_callback: Some(input_state_callback),
            video_refresh_callback: Some(video_refresh_callback),
        };

        for (key, value) in &values {
            print!("key -> {:?};", key);
            println!(" value -> {:?};", value);

            if key == "core" {
                let core_wrapper = libretro::core::load(value, &callbacks);

                match core_wrapper {
                    Ok(libretro) => {
                        let v = libretro.version();

                        println!("{:?}", v);

                        if v == 1 {
                            libretro.init();
                            libretro.de_init();
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    } else {
        println!("sem argumentos validos {:?}", values.len());
    }
}
