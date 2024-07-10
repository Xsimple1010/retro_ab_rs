use std::ptr::addr_of;
use std::sync::Arc;
use uuid::Uuid;
use crate::{
    core::CoreWrapper, environment::RetroEnvCallbacks,
    paths::Paths,
};
use crate::erro_handle::ErroHandle;

static mut CONTEXTS: Vec<Arc<RetroContext>> = Vec::new();

// #[derive(Debug, PartialEq, Eq)]
pub struct RetroContext {
    pub id: Uuid,
    pub core: Arc<CoreWrapper>,
}

impl Drop for RetroContext {
    fn drop(&mut self) {
        let _ = self.delete();
    }
}

impl RetroContext {
    pub fn new(path: &str,
               paths: Paths,
               callbacks: RetroEnvCallbacks) -> Result<Arc<RetroContext>, ErroHandle> {
        let context = Arc::new(RetroContext {
            id: Uuid::new_v4(),
            core: CoreWrapper::new(path, paths.clone(), callbacks),
        });

        context.core.init()?;

        unsafe {
            CONTEXTS.push(Arc::clone(&context));
        }

        Ok(context)
    }

    pub fn is_valid(&self) -> bool {
        let mut is_valide = false;

        unsafe {
            for ctx in &*addr_of!(CONTEXTS) {
                if ctx.id == self.id {
                    is_valide = true;
                    break;
                }
            }
        }

        is_valide
    }

    pub fn delete(&self) -> Result<(), ErroHandle> {
        unsafe {
            let position = CONTEXTS.partition_point(|ctx| ctx.id == self.id);

            if !CONTEXTS.is_empty() {
                CONTEXTS.remove(position - 1);
            }
        };

        self.core.de_init()?;

        Ok(())
    }

    pub fn get_num_contexts() -> usize {
        unsafe { CONTEXTS.len() }
    }
}


#[cfg(test)]
mod retro_context {
    use crate::erro_handle::ErroHandle;
    use crate::test_tools::context::get_context;

    #[test]
    fn test_create_and_delete() -> Result<(), ErroHandle> {
        let ctx = get_context()?;

        assert_eq!(
            ctx.is_valid(),
            true,
            "O contexto id -> {:?} nao foi inicializado!",
            ctx.id
        );

        let current_id = ctx.id.clone();

        ctx.delete()?;

        assert_eq!(
            ctx.is_valid(),
            false,
            "O contexto id -> {:?} nao foi removido!",
            current_id
        );

        Ok(())
    }
}
