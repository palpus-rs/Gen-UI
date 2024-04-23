use crate::prop::builtin::{draw_color::DrawColor, Layout};

#[derive(Debug,Clone,Default)]
pub struct ViewProps{
    pub draw_bg: DrawColor,
    pub show_bg: bool,
    pub layout: Layout
}