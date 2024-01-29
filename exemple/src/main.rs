use retro_ab::core;
use std::env;

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
    let value = retro_ab::args_manager::get_values(env::args().collect());

    let mut context: Option<&'static core::Context> = None;

    let callbacks = core::CoreCallbacks {
        audio_sample_batch_callback,
        audio_sample_callback,
        input_poll_callback,
        input_state_callback,
        video_refresh_callback,
    };

    match value.get_key_value("core") {
        Some((_, value)) => {
            let result = core::load(value, callbacks);

            match result {
                Ok(ctx) => context = Some(ctx),
                _ => {}
            }
        }
        _ => {}
    }

    match context {
        Some(ctx) => {
            println!("=======core context=======");
            println!("core version -> {:?}", core::version());
            println!("subsystem -> {:?}", ctx.core.borrow().use_subsystem);
            println!("pixel format -> {:?}", ctx.core.borrow().video.pixel_format);
            println!("language -> {:?}", ctx.core.borrow().language);

            println!("options version -> {:?}", ctx.options.borrow().version);
            // println!("options here\n");
            // for opt in &ctx.options.borrow().opts {
            //     println!("{:?}", opt.key);
            //     println!("{:?}", opt.desc);
            //     println!("{:?}", opt.desc_categorized);
            //     println!("{:?}", opt.info);
            //     println!("{:?}", opt.info_categorized);
            //     println!("{:?}", opt.default_value);
            //     println!("");
            // }
        }
        None => {}
    }
}
