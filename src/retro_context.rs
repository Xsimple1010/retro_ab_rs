use std::{path::PathBuf, sync::Arc};

use crate::{
    binding_libretro::LibretroRaw, core::CoreWrapper, environment::RetroEnvCallbacks,
    managers::option_manager::OptionManager, system,
};

// #[derive(Debug, PartialEq, Eq)]
pub struct RetroContext {
    pub id: String,
    pub core: CoreWrapper,
    pub callbacks: RetroEnvCallbacks,
    pub options: OptionManager,
}

static mut CONTEXTS: Vec<Arc<RetroContext>> = Vec::new();

pub fn delete(ctx_to_delete: Arc<RetroContext>) {
    unsafe {
        let position = CONTEXTS.partition_point(|ctx| ctx.id == ctx_to_delete.id);

        if !CONTEXTS.is_empty() {
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

pub fn create(raw: LibretroRaw, callbacks: RetroEnvCallbacks) -> Arc<RetroContext> {
    let sys_info = system::get_sys_info(&raw);

    let context = Arc::new(RetroContext {
        id: create_id(),
        core: CoreWrapper::new(raw),
        callbacks,
        options: OptionManager::new(
            PathBuf::from("cfg").join(sys_info.library_name.lock().unwrap().to_owned() + ".opt"),
        ),
    });

    *context.core.system.info.library_name.lock().unwrap() =
        sys_info.library_name.lock().unwrap().to_owned();

    *context.core.system.info.library_version.lock().unwrap() =
        sys_info.library_version.lock().unwrap().clone();

    *context.core.system.info.valid_extensions.lock().unwrap() =
        sys_info.valid_extensions.lock().unwrap().clone();

    *context.core.system.info.need_fullpath.lock().unwrap() =
        sys_info.need_fullpath.lock().unwrap().to_owned();

    *context.core.system.info.block_extract.lock().unwrap() =
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
}
