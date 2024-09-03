use crate::retro_sys::retro_hw_context_type;

#[derive(Debug)]
pub struct GraphicApi {
    pub api: retro_hw_context_type,
    pub fbo: Option<usize>,
    pub stencil: Option<usize>,
}
impl GraphicApi {
    pub fn new() -> Self {
        Self {
            api: retro_hw_context_type::RETRO_HW_CONTEXT_OPENGL,
            fbo: None,
            stencil: None,
        }
    }
}
