use crate::paths::Paths;

pub fn get_paths() -> Paths {
    Paths {
        save: "retro_out_test/save".to_string(),
        system: "retro_out_test/system".to_string(),
    }
}
