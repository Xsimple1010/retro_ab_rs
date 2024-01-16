use super::binding_libretro::LibretroRaw;

pub fn load(path: &String) -> Result<LibretroRaw, ::libloading::Error> {
    unsafe {
        let result = LibretroRaw::new(path);

        match result {
            Ok(libretro_raw) => Ok(libretro_raw),
            Err(e) => Err(e),
        }
    }
}
