extern crate sdl2;
mod args_manager;
mod libretro;

fn main() {
    let values = args_manager::get_values();

    if !values.is_empty() {
        for (key, value) in &values {
            print!("key -> {:?};", key);
            println!(" value -> {:?};", value);

            if key == "core" {
                let libretro_raw = libretro::core::load(value);

                match libretro_raw {
                    Ok(libretro) => unsafe {
                        let v = libretro.retro_api_version();
                        println!("{:?}", v);
                    },
                    Err(_) => {}
                }
            }
        }
    } else {
        println!("sem argumentos validos {:?}", values.len());
    }
}
