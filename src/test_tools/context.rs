use std::sync::Arc;

use crate::{binding::binding_libretro::LibretroRaw, core::RetroContext, retro_context};

use super::{core, paths};

pub fn get_context(raw: LibretroRaw) -> Arc<RetroContext> {
    retro_context::create(raw, paths::get_paths(), core::get_callbacks())
}
