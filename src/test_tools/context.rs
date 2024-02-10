use std::sync::Arc;

use crate::{core::RetroContext, libretro::binding_libretro::LibretroRaw, retro_context};

use super::{core, paths};

pub fn get_context(raw: LibretroRaw) -> Arc<RetroContext> {
    retro_context::create(raw, paths::get_paths(), core::get_callbacks())
}
