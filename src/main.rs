use std::rc::Rc;

use libretro::core::{CoreCallbacks, CoreWrapper};

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
    let callbacks: CoreCallbacks = CoreCallbacks {
        audio_sample_callback: audio_sample_callback,
        audio_sample_batch_callback: audio_sample_batch_callback,
        input_poll_callback: input_poll_callback,
        input_state_callback: input_state_callback,
        video_refresh_callback: video_refresh_callback,
    };

    let mut core_wrapper: Option<Rc<CoreWrapper>> = None;

    if values.contains_key("core") {
        let value = values.get("core");

        match value {
            Some(path) => {
                let result = libretro::core::load(path, callbacks);

                match result {
                    Ok(core) => {
                        core_wrapper = Some(core);

                        let v = core_wrapper
                            .as_ref()
                            .expect("erro ao reconhecer a versao do core")
                            .version();

                        println!("core version -> {:?}", v);
                    }
                    Err(e) => println!("{e}"),
                }
            }
            _ => {}
        }
    }

    if values.contains_key("rom") {
        let value = values.get("core");

        match value {
            Some(path) => {
                let v = core_wrapper
                    .as_ref()
                    .expect("erro ao carrega a rom")
                    .version();
                println!("core version -> {:?}", v);
            }
            _ => {}
        }
    }
}
