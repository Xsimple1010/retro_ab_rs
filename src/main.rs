extern crate sdl2;
mod args_manager;
mod libretro;

extern  crate rust_libretro_sys;

fn main() {

    let values = args_manager::get_values();

    if !values.is_empty() {
        for arg in &values  {
            print!("key -> {:?};", arg.0);
            println!(" value -> {:?};", arg.1);

            if arg.0 == "core" {
                let core = libretro::core::load(arg.1);

                let version = core.api_version();
                println!("{:?}", version);
            }

        }
    } else {
        println!("sem argumentos validos {:?}", values.len());
    }

}
