use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    binding_libretro::{retro_system_info, LibretroRaw},
    core::{CoreCallbacks, CoreWrapper, SysInfo},
    option_manager::OptionManager,
    tools::ffi_tools::get_str_from_ptr,
};

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RetroContext {
    pub id: String,
    pub core: CoreWrapper,
    pub callbacks: CoreCallbacks,
    pub options: OptionManager,
}

pub fn get_sys_info(raw: &LibretroRaw) -> SysInfo {
    unsafe {
        let sys_info = &mut retro_system_info {
            block_extract: false,
            need_fullpath: false,
            library_name: "".as_ptr() as *const i8,
            library_version: "".as_ptr() as *const i8,
            valid_extensions: "".as_ptr() as *const i8,
        };

        raw.retro_get_system_info(sys_info);

        SysInfo {
            library_name: Mutex::new(get_str_from_ptr(sys_info.library_name)),
            library_version: Mutex::new(get_str_from_ptr(sys_info.library_version)),
            valid_extensions: Mutex::new(get_str_from_ptr(sys_info.valid_extensions)),
            need_fullpath: Mutex::new(sys_info.need_fullpath),
            block_extract: Mutex::new(sys_info.block_extract),
        }
    }
}

static mut CONTEXTS: Vec<Arc<RetroContext>> = Vec::new();

pub fn _delete(_ctx: Arc<RetroContext>) {}

pub fn get_num_context() -> usize {
    unsafe { CONTEXTS.len() }
}

fn create_id() -> String {
    "".to_string()
}

pub fn create(raw: LibretroRaw, callbacks: CoreCallbacks) -> Arc<RetroContext> {
    let sys_info = get_sys_info(&raw);

    let context = Arc::new(RetroContext {
        id: create_id(),
        core: CoreWrapper::new(raw),
        callbacks,
        options: OptionManager::new(
            PathBuf::from("cfg").join(sys_info.library_name.lock().unwrap().to_owned() + ".opt"),
        ),
    });

    *context.core.sys_info.library_name.lock().unwrap() =
        sys_info.library_name.lock().unwrap().to_owned();

    *context.core.sys_info.library_version.lock().unwrap() =
        sys_info.library_version.lock().unwrap().clone();

    *context.core.sys_info.valid_extensions.lock().unwrap() =
        sys_info.valid_extensions.lock().unwrap().clone();

    *context.core.sys_info.need_fullpath.lock().unwrap() =
        sys_info.need_fullpath.lock().unwrap().to_owned();

    *context.core.sys_info.block_extract.lock().unwrap() =
        sys_info.block_extract.lock().unwrap().to_owned();

    unsafe {
        CONTEXTS.push(Arc::clone(&context));
    }

    context
}
