use retro_ab::erro_handle::ErroHandle;
use retro_ab::retro_context::RetroContext;
use retro_ab::test_tools;
use std::sync::Arc;

pub fn setup() -> Result<Arc<RetroContext>, ErroHandle> {
    RetroContext::new(
        test_tools::constants::CORE_TEST_RELATIVE_PATH,
        test_tools::paths::get_paths(),
        test_tools::core::get_callbacks(),
    )
}
