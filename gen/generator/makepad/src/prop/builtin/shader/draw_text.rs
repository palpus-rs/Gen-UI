use crate::prop::builtin::{Font, TextWrap, Vec4};

#[derive(Clone, Default, Debug)]
pub struct DrawText {
    //pub geometry: GeometryQuad2D,
    pub text_style: TextStyle,
    pub wrap: Option<TextWrap>,
    pub ignore_newlines: Option<bool>,
    pub combine_spaces: Option<bool>,
    pub font_scale: Option<f64>,
    pub draw_depth: Option<f32>,
    pub color: Option<Vec4>,
}

#[derive(Clone, Debug, Default)]
pub struct TextStyle {
    pub font: Option<Font>,
    pub font_size: Option<f64>,
    pub brightness: Option<f32>,
    pub curve: Option<f32>,
    pub line_spacing: Option<f64>,
    pub top_drop: Option<f64>,
    pub height_factor: Option<f64>,
}
