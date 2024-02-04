use std::sync::Arc;

use retro_ab::{core, test_tools};

pub fn setup() -> Result<Arc<core::RetroContext>, String> {
    core::load(
        test_tools::constants::CORE_TEST_RELATIVE_PATH,
        test_tools::core::get_callbacks(),
    )
}
