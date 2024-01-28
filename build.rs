use std::{fs::OpenOptions, io::Read, os::windows::fs::FileExt, path::PathBuf};

fn main() {
    let out_path = PathBuf::from("./src/").join("binding_libretro.rs");

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
        .write_to_file(out_path.clone());

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("src/binding_libretro.rs")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents = "#![allow(dead_code,non_snake_case,non_camel_case_types,non_upper_case_globals)]\n"
        .to_owned()
        + &contents;

    file.seek_write(contents.as_bytes(), 0).unwrap();
}
