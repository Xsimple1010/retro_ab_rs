use crate::binding_libretro::{retro_game_info, retro_system_info};

use super::binding_libretro::LibretroRaw;

pub fn load(raw: &LibretroRaw, path: &String) {
    let sys_av_info: *mut retro_system_info = &mut retro_system_info {
        block_extract: false,
        need_fullpath: false,
        library_name: "".as_ptr() as *const i8,
        library_version: "".as_ptr() as *const i8,
        valid_extensions: "".as_ptr() as *const i8,
    };

    let _game_info: *mut retro_game_info = &mut retro_game_info {
        data: std::ptr::null(),
        meta: "".as_ptr() as *const i8,
        path: path.as_ptr() as *const i8,
        size: 0,
    };

    unsafe {
        raw.retro_get_system_info(sys_av_info);

        if sys_av_info.read().need_fullpath {
            println!("full path sim");
        }
    }
}
