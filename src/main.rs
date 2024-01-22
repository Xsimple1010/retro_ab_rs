use libretro::core::CoreCallbacks;

// extern crate sdl2;
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
    let callbacks: CoreCallbacks = CoreCallbacks {
        audio_sample_callback: audio_sample_callback,
        audio_sample_batch_callback: audio_sample_batch_callback,
        input_poll_callback: input_poll_callback,
        input_state_callback: input_state_callback,
        video_refresh_callback: video_refresh_callback,
    };

    let mut _core_ctx: Option<&'static libretro::environment::Context> = None;

    if values.contains_key("core") {
        let value = values.get("core");

        match value {
            Some(path) => {
                let result = libretro::core::load(path, callbacks);

                match result {
                    Ok(core) => {
                        _core_ctx = Some(core);

                        let v = libretro::core::version();

                        println!("core version -> {:?}", v);

                        libretro::core::init();
                    }
                    Err(e) => println!("{e}"),
                }
            }
            _ => {}
        }
    }

    match _core_ctx {
        Some(ctx) => {
            println!(
                "core is using a subsystem -> {:?}",
                ctx.core.borrow().use_subsystem
            );
        }
        None => {}
    }

    if values.contains_key("rom") {
        let value = values.get("core");

        match value {
            Some(_path) => {
                libretro::core::load_game(_path.to_owned());
            }
            _ => {}
        }
    }

    libretro::core::de_init();
}
