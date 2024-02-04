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

// #[derive(Debug, PartialEq, Eq)]
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

pub fn delete(ctx_to_delete: Arc<RetroContext>) {
    unsafe {
        let position = CONTEXTS.partition_point(|ctx| ctx.id == ctx_to_delete.id);

        if CONTEXTS.len() > 0 {
            CONTEXTS.remove(position - 1);
        }
    };
}

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

#[cfg(test)]
mod retro_context {
    use crate::{retro_context, test_tools};

    #[test]
    fn test_create_and_delete() {
        let raw_result = test_tools::core::get_raw();

        match raw_result {
            Ok(raw) => {
                let current_ctx = retro_context::create(raw, test_tools::core::get_callbacks());

                let len = retro_context::get_num_context();
                assert_eq!(
                    len, 1,
                    "há {:?} contextos ativos, a quantidade esperada era 1!",
                    len
                );

                retro_context::delete(current_ctx);
                let len = retro_context::get_num_context();
                assert_eq!(
                    len, 0,
                    "há {:?} contextos ativos, a quantidade esperada era 0!",
                    len
                );
            }
            _ => panic!("Não foi possível iniciar o núcleo"),
        };
    }

    #[test]
    fn test_get_sys_info() {
        let raw_result = test_tools::core::get_raw();

        match raw_result {
            Ok(raw) => {
                let sys_info = retro_context::get_sys_info(&raw);

                assert_eq!(
                    *sys_info.library_name.lock().unwrap().clone(),
                    "Snes9x".to_owned()
                );

                assert_eq!(
                    *sys_info.library_version.lock().unwrap().clone(),
                    "1.62.3 46f8a6b".to_owned()
                );

                assert_eq!(
                    *sys_info.valid_extensions.lock().unwrap().clone(),
                    "smc|sfc|swc|fig|bs|st".to_owned()
                );

                assert_eq!(*sys_info.block_extract.lock().unwrap(), false);

                assert_eq!(*sys_info.need_fullpath.lock().unwrap(), false);
            }
            _ => {}
        }
    }
}
