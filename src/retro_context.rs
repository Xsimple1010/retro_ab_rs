use std::{cell::RefCell, path::PathBuf};

use crate::{
    binding_libretro::{retro_system_info, LibretroRaw},
    core::{CoreCallbacks, CoreWrapper, SysInfo},
    ffi_tools,
    option_manager::OptionManager,
};

pub struct RetroContext {
    pub core: RefCell<CoreWrapper>,
    pub callbacks: RefCell<CoreCallbacks>,
    pub options: RefCell<OptionManager>,
}

pub fn get_sys_info(raw: &LibretroRaw) -> SysInfo {
    unsafe {
        let sys_info: *mut retro_system_info = &mut retro_system_info {
            block_extract: false,
            need_fullpath: false,
            library_name: "".as_ptr() as *const i8,
            library_version: "".as_ptr() as *const i8,
            valid_extensions: "".as_ptr() as *const i8,
        };

        raw.retro_get_system_info(sys_info);

        let sys_info = *(sys_info as *mut retro_system_info);

        SysInfo {
            library_name: ffi_tools::get_str_from_ptr(sys_info.library_name),
            library_version: ffi_tools::get_str_from_ptr(sys_info.library_version),
            valid_extensions: ffi_tools::get_str_from_ptr(sys_info.valid_extensions),
            need_fullpath: sys_info.need_fullpath,
            block_extract: sys_info.block_extract,
        }
    }
}

pub fn create(raw: &LibretroRaw, callbacks: CoreCallbacks) -> RetroContext {
    let sys_info = get_sys_info(raw);

    let context = RetroContext {
        core: RefCell::new(CoreWrapper::new()),
        callbacks: RefCell::new(callbacks),
        options: RefCell::new(OptionManager::new(
            PathBuf::from("cfg").join(sys_info.library_name.clone() + ".opt"),
        )),
    };

    context.core.borrow_mut().sys_info.library_name = sys_info.library_name;
    context.core.borrow_mut().sys_info.library_version = sys_info.library_version;
    context.core.borrow_mut().sys_info.valid_extensions = sys_info.valid_extensions;
    context.core.borrow_mut().sys_info.need_fullpath = sys_info.need_fullpath;
    context.core.borrow_mut().sys_info.block_extract = sys_info.block_extract;

    context
}
