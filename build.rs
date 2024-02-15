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

fn libretro_tool(_out_path: &PathBuf) {
    // Compile the C library
    cc::Build::new()
        .file("src/libretro/log_interface.c")
        .compile("log_interface");

    // Generate bindings
    // let bindings = bindgen::Builder::default()
    //     .header("src/libretro/log_interface.h")
    //     .allowlist_function(
    //         "configure_log_interface|set_variable_value_as_null|set_new_value_variable|set_directory",
    //     )
    //     .allowlist_item("rs_cb_t")
    //     .clang_arg("-fparse-all-comments")
    //     .default_enum_style(bindgen::EnumVariation::Rust {
    //         non_exhaustive: true,
    //     })
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.

    // bindings
    //     .write_to_file(out_path.join("binding_log_interface.rs"))
    //     .expect("Couldn't write bindings!");
}

fn core_bindings(out_path: &PathBuf) {
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

    configure_files(temp_path, out_path.join("binding_libretro.rs"));
}

fn main() {
    let out_path = PathBuf::from("./src/binding");
    core_bindings(&out_path);
    libretro_tool(&out_path);
}
