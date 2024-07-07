use crate::prop::builtin::{draw_quad::DrawQuad, Layout, Walk};

#[derive(Debug, Clone, Default)]
pub struct ShaderProps{
    pub draw_shader: Option<DrawQuad>,
    pub walk: Option<Walk>,
    pub layout: Option<Layout>,
    pub time: Option<f32>,
    // #[rust] next_frame: NextFrame,
}

