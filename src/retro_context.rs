use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    binding_libretro::{retro_system_info, LibretroRaw},
    core::{CoreCallbacks, CoreWrapper, SysInfo},
    ffi_tools,
    option_manager::OptionManager,
};

pub struct RetroContext {
    pub core: CoreWrapper,
    pub callbacks: CoreCallbacks,
    pub options: OptionManager,
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
            library_name: RefCell::new(ffi_tools::get_str_from_ptr(sys_info.library_name)),
            library_version: RefCell::new(ffi_tools::get_str_from_ptr(sys_info.library_version)),
            valid_extensions: RefCell::new(ffi_tools::get_str_from_ptr(sys_info.valid_extensions)),
            need_fullpath: RefCell::new(sys_info.need_fullpath),
            block_extract: RefCell::new(sys_info.block_extract),
        }
    }
}

pub fn create(raw: &LibretroRaw, callbacks: CoreCallbacks) -> Rc<RetroContext> {
    let sys_info = get_sys_info(raw);

    let context = Rc::new(RetroContext {
        core: CoreWrapper::new(),
        callbacks: callbacks,
        options: OptionManager::new(
            PathBuf::from("cfg").join(sys_info.library_name.borrow().clone() + ".opt"),
        ),
    });

    *context.core.sys_info.library_name.borrow_mut() = sys_info.library_name.borrow().clone();
    *context.core.sys_info.library_version.borrow_mut() = sys_info.library_version.borrow().clone();
    *context.core.sys_info.valid_extensions.borrow_mut() =
        sys_info.valid_extensions.borrow().clone();
    *context.core.sys_info.need_fullpath.borrow_mut() = sys_info.need_fullpath.borrow().clone();
    *context.core.sys_info.block_extract.borrow_mut() = sys_info.block_extract.borrow().clone();

    context
}
