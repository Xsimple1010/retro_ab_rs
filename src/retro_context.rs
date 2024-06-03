use std::{path::PathBuf, sync::Arc};

use uuid::Uuid;

use crate::{
    binding::binding_libretro::LibretroRaw, core::CoreWrapper, environment::RetroEnvCallbacks,
    managers::option_manager::OptionManager, paths::Paths, system,
};

// #[derive(Debug, PartialEq, Eq)]
pub struct RetroContext {
    pub id: Uuid,
    pub core: CoreWrapper,
    pub callbacks: RetroEnvCallbacks,
    pub options: OptionManager,
    pub paths: Paths,
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

fn create_id() -> Uuid {
    Uuid::new_v4()
}

pub fn create(raw: LibretroRaw, paths: Paths, callbacks: RetroEnvCallbacks) -> Arc<RetroContext> {
    let sys_info = system::get_sys_info(&raw);

    let options = OptionManager::new(
        PathBuf::from(paths.opt.clone())
            .join(sys_info.library_name.lock().unwrap().to_owned() + ".opt"),
    );

    let context = Arc::new(RetroContext {
        id: create_id(),
        core: CoreWrapper::new(raw),
        callbacks,
        options,
        paths,
    });

    sys_info
        .library_name
        .lock()
        .unwrap()
        .clone_into(&mut context.core.system.info.library_name.lock().unwrap());

    context
        .core
        .system
        .info
        .library_version
        .lock()
        .unwrap()
        .clone_from(&sys_info.library_version.lock().unwrap());

    context
        .core
        .system
        .info
        .valid_extensions
        .lock()
        .unwrap()
        .clone_from(&sys_info.valid_extensions.lock().unwrap());

    sys_info
        .need_fullpath
        .lock()
        .unwrap()
        .clone_into(&mut context.core.system.info.need_fullpath.lock().unwrap());

    sys_info
        .block_extract
        .lock()
        .unwrap()
        .clone_into(&mut context.core.system.info.block_extract.lock().unwrap());

    unsafe {
        CONTEXTS.push(Arc::clone(&context));
    }

    context
}

#[cfg(test)]
mod retro_context {
    use libloading::Error;
    use std::{ptr::addr_of, sync::Arc};
    use uuid::Uuid;

    use crate::{retro_context, test_tools};

    use super::{RetroContext, CONTEXTS};

    fn create_ctx() -> Result<Arc<RetroContext>, Error> {
        let raw_result = test_tools::core::get_raw();

        match raw_result {
            Ok(raw) => {
                let ctx = retro_context::create(
                    raw,
                    test_tools::paths::get_paths(),
                    test_tools::core::get_callbacks(),
                );

                Ok(ctx)
            }
            Err(e) => Err(e),
        }
    }

    fn has_initialized(id: Uuid) -> bool {
        let mut has_initialized = false;

        unsafe {
            for ctx in &*addr_of!(CONTEXTS) {
                if ctx.id == id {
                    has_initialized = true;
                }
            }
        }

        has_initialized
    }

    #[test]
    fn test_create_and_delete() -> Result<(), Error> {
        let ctx = create_ctx()?;

        assert_eq!(
            has_initialized(ctx.id),
            true,
            "O contexto id -> {:?} nao foi inicializado!",
            ctx.id
        );

        let current_id = ctx.id.clone();
        retro_context::delete(ctx);

        assert_eq!(
            has_initialized(current_id),
            false,
            "O contexto id -> {:?} nao foi removido!",
            current_id
        );

        Ok(())
    }
}
