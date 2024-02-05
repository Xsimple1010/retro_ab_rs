use retro_ab::core;
use std::{env, sync::Arc};

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

    let mut context: Option<Arc<core::RetroContext>> = None;

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

    match value.get_key_value("rom") {
        Some((_, _value)) => match &context {
            Some(_ctx) => {
                // core::init(&ctx);
                // core::load_game(ctx, value);
                // core::run(&ctx);
            }
            _ => {}
        },
        _ => {}
    }

    match context {
        Some(ctx) => {
            println!("=======core context=======");
            println!("core version -> {:?}", core::version(&ctx));
            println!("subsystem -> {:?}", *ctx.core.use_subsystem.lock().unwrap());
            println!(
                "pixel format -> {:?}",
                *ctx.core.video.pixel_format.lock().unwrap()
            );
            println!("language -> {:?}", *ctx.core.language.lock().unwrap());

            println!("\n+++++sys info here+++++");
            println!(
                "library_name -> {:?}",
                ctx.core.sys_info.library_name.lock().unwrap()
            );
            println!(
                "library_version -> {:?}",
                ctx.core.sys_info.library_version.lock().unwrap()
            );
            println!(
                "valid_extensions -> {:?}",
                ctx.core.sys_info.valid_extensions.lock().unwrap()
            );
            println!(
                "need_fullpath -> {:?}",
                ctx.core.sys_info.need_fullpath.lock().unwrap()
            );
            println!(
                "block_extract -> {:?}",
                ctx.core.sys_info.block_extract.lock().unwrap()
            );

            println!("\n+++++options here+++++");
            println!(
                "file path -> {:?} \n",
                ctx.options.file_path.lock().unwrap()
            );
            for opt in &*ctx.options.opts.lock().unwrap() {
                println!("key -> {:?}", opt.key.lock().unwrap());
                println!("visibility -> {:?}", opt.visibility.lock().unwrap());
                println!(
                    "desc_categorized -> {:?}",
                    opt.desc_categorized.lock().unwrap()
                );
                println!("info -> {:?}", opt.info.lock().unwrap());
                println!(
                    "info_categorized -> {:?}",
                    opt.info_categorized.lock().unwrap()
                );
                println!("default_value -> {:?}", opt.default_value.lock().unwrap());
                println!("");
            }

            println!("\n+++++categories here+++++");
            for category in &*ctx.options.categories.lock().unwrap() {
                println!("key -> {:?}", category.key.lock().unwrap());

                println!("info -> {:?}", category.info.lock().unwrap());

                println!("desc -> {:?}", category.desc.lock().unwrap());
                println!("");
            }

            println!("\n+++++controller info+++++");
            for ctr_info in &*ctx.core.controller_info.lock().unwrap() {
                println!("num_types -> {:?}", ctr_info.num_types.lock().unwrap());

                for desc in &ctr_info.types {
                    println!("id -> {:?}", desc.id.lock().unwrap());
                    println!("desc -> {:?}", desc.desc.lock().unwrap());
                }

                println!("")
            }

            core::de_init(ctx);
        }
        None => {}
    }
}
