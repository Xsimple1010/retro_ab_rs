use libretro::core::CoreWrapper;

extern crate sdl2;
mod args_manager;
mod libretro;

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

fn main() {
    let values = args_manager::get_values();

    if !values.is_empty() {
        let mut core_wrapper: Option<&CoreWrapper> = None;

        for (key, value) in &values {
            print!("key -> {:?};", key);
            println!(" value -> {:?};", value);

            if key == "core" {
                let callbacks = libretro::core::CoreCallbacks {
                    audio_sample_callback: audio_sample_callback,
                    audio_sample_batch_callback: audio_sample_batch_callback,
                    input_poll_callback: input_poll_callback,
                    input_state_callback: input_state_callback,
                    video_refresh_callback: video_refresh_callback,
                };

                let result = libretro::core::load(value, callbacks);
                match result {
                    Ok(libretro) => {
                        core_wrapper = Some(libretro);
                        let v = core_wrapper.expect("erro").version();

                        println!("{:?}", v);

                        if v == 1 {
                            core_wrapper.expect("erro").init();
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }

            if key == "rom" {
                core_wrapper.expect("msg").load_game(value.to_owned());
            }
        }

        core_wrapper.expect("msg").de_init();
    } else {
        println!("sem argumentos validos {:?}", values.len());
    }
}
