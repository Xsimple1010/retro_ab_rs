use std::sync::Arc;
use crate::erro_handle::ErroHandle;
use crate::retro_context::RetroContext;
use crate::test_tools::constants::CORE_TEST_RELATIVE_PATH;
use super::{core, paths};

pub fn get_context() -> Result<Arc<RetroContext>, ErroHandle> {
    RetroContext::new(CORE_TEST_RELATIVE_PATH, paths::get_paths(), core::get_callbacks())
}
