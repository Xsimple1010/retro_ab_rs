extern crate sdl2;
mod args_manager;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let values = args_manager::get_values(&args);

    if !values.is_empty() {
        for arg in &values  {
            print!("key -> {:?};", arg.0);
            println!(" value -> {:?};", arg.1)
        }
    } else {
        println!("sem argumentos validos {:?}", values.len());
    }

}
