use retro_ab::{core, test_tools};
use std::{env, f32::consts::E, sync::Arc};

fn main() {
    let value = retro_ab::args_manager::get_values(env::args().collect());

    let mut context: Option<Arc<core::RetroContext>> = None;

    match value.get_key_value("core") {
        Some((_, value)) => {
            let result = core::load(
                value,
                test_tools::paths::get_paths(),
                test_tools::core::get_callbacks(),
            );

            match result {
                Ok(ctx) => context = Some(ctx),
                _ => {}
            }
        }
        _ => {}
    }

    match value.get_key_value("rom") {
        Some((_, value)) => match &context {
            Some(ctx) => {
                core::init(&ctx);
                match core::load_game(ctx, value) {
                    Ok(state) => println!("game is loaded -> {:?}", state),
                    Err(e) => {
                        println!("[erro]: level:{:?}; message: {:?}", e.level, e.message)
                    }
                };
                core::run(&ctx);
            }
            _ => {}
        },
        _ => {}
    }

    match context {
        Some(ctx) => {
            println!("=======core context=======");
            println!("core version -> {:?}", core::version(&ctx));
            // println!("subsystem -> {:?}", *ctx.core.use_subsystem.lock().unwrap());
            println!(
                "pixel format -> {:?}",
                *ctx.core.video.pixel_format.lock().unwrap()
            );
            println!("language -> {:?}", *ctx.core.language.lock().unwrap());

            println!("\n+++++sys info here+++++");
            println!(
                "library_name -> {:?}",
                ctx.core.system.info.library_name.lock().unwrap()
            );
            println!(
                "library_version -> {:?}",
                ctx.core.system.info.library_version.lock().unwrap()
            );
            println!(
                "valid_extensions -> {:?}",
                ctx.core.system.info.valid_extensions.lock().unwrap()
            );
            println!(
                "need_fullpath -> {:?}",
                ctx.core.system.info.need_fullpath.lock().unwrap()
            );
            println!(
                "block_extract -> {:?}",
                ctx.core.system.info.block_extract.lock().unwrap()
            );

            println!("\n+++++options here+++++");
            println!(
                "file path -> {:?} \n",
                ctx.options.file_path.lock().unwrap()
            );
            // for opt in &*ctx.options.opts.lock().unwrap() {
            //     println!("key -> {:?}", opt.key.lock().unwrap());
            //     println!("visibility -> {:?}", opt.visibility.lock().unwrap());
            //     println!(
            //         "desc_categorized -> {:?}",
            //         opt.desc_categorized.lock().unwrap()
            //     );
            //     println!("info -> {:?}", opt.info.lock().unwrap());
            //     println!(
            //         "info_categorized -> {:?}",
            //         opt.info_categorized.lock().unwrap()
            //     );
            //     println!("default_value -> {:?}", opt.default_value.lock().unwrap());
            //     println!("");
            // }

            // println!("\n+++++categories here+++++");
            // for category in &*ctx.options.categories.lock().unwrap() {
            //     println!("key -> {:?}", category.key.lock().unwrap());

            //     println!("info -> {:?}", category.info.lock().unwrap());

            //     println!("desc -> {:?}", category.desc.lock().unwrap());
            //     println!("");
            // }

            // println!("\n+++++controller info+++++");
            // for ctr_info in &*ctx.core.system.ports.lock().unwrap() {
            //     println!("num_types -> {:?}", ctr_info.num_types.lock().unwrap());

            //     for desc in &ctr_info.types {
            //         println!("id -> {:?}", desc.id.lock().unwrap());
            //         println!("desc -> {:?}", desc.desc.lock().unwrap());
            //     }

            //     println!("")
            // }

            // println!("\n+++++system+++++");
            // for subsystem in &*ctx.core.system.subsystem.lock().unwrap() {
            //     println!("id -> {:?}", subsystem.id.lock().unwrap());
            //     println!("ident -> {:?}", subsystem.ident.lock().unwrap());
            //     println!("desc -> {:?}", subsystem.desc.lock().unwrap());

            //     for rom in &*subsystem.roms.lock().unwrap() {
            //         println!("rom: desc -> {:?}", rom.desc.lock().unwrap());
            //         println!(
            //             "rom: valide extensions -> {:?}",
            //             rom.valid_extensions.lock().unwrap()
            //         );

            //         println!(
            //             "memory: extensions -> {:?}",
            //             rom.memory.extension.lock().unwrap()
            //         );

            //         println!("memory: type -> {:?}", rom.memory.type_.lock().unwrap());
            //     }

            //     println!("")
            // }

            core::de_init(ctx);
        }
        None => {}
    }
}
