use retro_ab::retro_sys::retro_hw_context_type;
use retro_ab::test_tools;
use retro_ab::{erro_handle::ErroHandle, retro_ab::RetroAB};
use std::env;

fn main() -> Result<(), ErroHandle> {
    let value = retro_ab::args_manager::get_values(env::args().collect());

    let mut context: Option<RetroAB> = None;

    match value.get_key_value("core") {
        Some((_, value)) => {
            context = Some(RetroAB::new(
                value,
                test_tools::paths::get_paths()?,
                test_tools::core::get_callbacks(),
                retro_hw_context_type::RETRO_HW_CONTEXT_NONE,
            )?);
        }
        _ => {}
    }

    match value.get_key_value("rom") {
        Some((_, value)) => match &context {
            Some(ctx) => {
                ctx.core().init()?;
                let state = ctx.core().load_game(value)?;

                println!("loaded -> {:?}", state);
                ctx.core().run()?;
            }
            _ => {}
        },
        _ => {}
    }

    match &context {
        Some(ctx) => {
            println!("=======core context=======");
            // println!("core version -> {:?}", ctx.core());
            println!(
                "pixel format -> {:?}",
                *ctx.core().av_info.video.pixel_format.lock().unwrap()
            );
            println!(
                "base_height -> {:?}",
                *ctx.core()
                    .av_info
                    .video
                    .geometry
                    .base_height
                    .lock()
                    .unwrap()
            );
            println!(
                "base_width -> {:?}",
                *ctx.core().av_info.video.geometry.base_width.lock().unwrap()
            );
            println!(
                "aspect_ratio -> {:?}",
                *ctx.core()
                    .av_info
                    .video
                    .geometry
                    .aspect_ratio
                    .lock()
                    .unwrap()
            );
            println!("language -> {:?}", *ctx.core().language.lock().unwrap());

            println!("\n+++++sys info here+++++");
            println!(
                "library_name -> {:?}",
                ctx.core().system.info.library_name.lock().unwrap()
            );
            println!(
                "library_version -> {:?}",
                ctx.core().system.info.library_version.lock().unwrap()
            );
            println!(
                "valid_extensions -> {:?}",
                ctx.core().system.info.valid_extensions.lock().unwrap()
            );
            println!(
                "need_fullpath -> {:?}",
                ctx.core().system.info.need_full_path.lock().unwrap()
            );
            println!(
                "block_extract -> {:?}",
                ctx.core().system.info.block_extract.lock().unwrap()
            );

            println!("\n+++++options here+++++");
            println!(
                "file path -> {:?} \n",
                ctx.core().options.file_path.lock().unwrap()
            );
            // for opt in &*ctx.core().options.opts.lock().unwrap() {
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
            //
            // println!("\n+++++categories here+++++");
            // for category in &*ctx.core().options.categories.lock().unwrap() {
            //     println!("key -> {:?}", category.key.lock().unwrap());
            //
            //     println!("info -> {:?}", category.info.lock().unwrap());
            //
            //     println!("desc -> {:?}", category.desc.lock().unwrap());
            //     println!("");
            // }
            //
            // println!("\n+++++controller info+++++");
            // for ctr_info in &*ctx.core().system.ports.lock().unwrap() {
            //     println!("num_types -> {:?}", ctr_info.num_types.lock().unwrap());
            //
            //     for desc in &ctr_info.types {
            //         println!("id -> {:?}", desc.id.lock().unwrap());
            //         println!("desc -> {:?}", desc.desc.lock().unwrap());
            //     }
            //
            //     println!("")
            // }

            println!("\n+++++system+++++");
            for subsystem in &*ctx.core().system.subsystem.lock().unwrap() {
                println!("id -> {:?}", subsystem.id.lock().unwrap());
                println!("ident -> {:?}", subsystem.ident.lock().unwrap());
                println!("desc -> {:?}", subsystem.desc.lock().unwrap());

                for rom in &*subsystem.roms.lock().unwrap() {
                    println!("rom: desc -> {:?}", rom.desc.lock().unwrap());
                    println!(
                        "rom: valide extensions -> {:?}",
                        rom.valid_extensions.lock().unwrap()
                    );

                    println!(
                        "memory: extensions -> {:?}",
                        rom.memory.extension.lock().unwrap()
                    );

                    println!("memory: type -> {:?}", rom.memory.type_.lock().unwrap());
                }
            }

            println!("\n+++++video+++++");
            println!(
                "context_type -> {:?}",
                ctx.core().av_info.video.graphic_api.context_type
            );
            println!(
                "pixel_format -> {:?}",
                ctx.core().av_info.video.pixel_format.lock().unwrap()
            );
            println!(
                "max_height -> {:?}",
                ctx.core().av_info.video.geometry.max_height.lock().unwrap()
            );
            println!(
                "max_width -> {:?}",
                ctx.core().av_info.video.geometry.max_width.lock().unwrap()
            );
        }
        None => {}
    }

    Ok(())
}
