use std::{
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

fn configure_files(temp_path: PathBuf, out_path: PathBuf) {
    let mut temp_file = OpenOptions::new().read(true).open(temp_path).unwrap();

    let mut temp_contents = String::new();
    temp_file.read_to_string(&mut temp_contents).unwrap();

    let mut bindings_file = File::create(out_path).unwrap();

    bindings_file
        .write_all(
            b"#![allow(dead_code,non_snake_case,non_camel_case_types,non_upper_case_globals)]\n\n",
        )
        .unwrap();

    bindings_file.write_all(temp_contents.as_bytes()).unwrap();
}

fn main() {
    let out_path = PathBuf::from("./src/libretro").join("binding_libretro.rs");
    let temp_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("temp_binding_libretro.rs");

    let _ = bindgen::Builder::default()
        .header("src/libretro/libretro.h")
        .clang_arg("-I.")
        .allowlist_type("(retro|RETRO)_.*")
        .allowlist_function("(retro|RETRO)_.*")
        .allowlist_var("(retro|RETRO)_.*")
        .prepend_enum_name(false)
        .impl_debug(true)
        .clang_arg("-fparse-all-comments")
        .enable_function_attribute_detection()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .dynamic_link_require_all(true)
        .dynamic_library_name("LibretroRaw")
        .newtype_enum("retro_key")
        .bitfield_enum("retro_mod")
        .generate()
        .expect("Unable to generate libretro.h bindings")
        .write_to_file(temp_path.clone());

    configure_files(temp_path, out_path);
}
