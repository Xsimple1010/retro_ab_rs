use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from("./src/libretro/");

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
        .write_to_file(out_path.join("binding_libretro.rs"));
}
