use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::widget::utils::f32_prop;

use super::draw_quad::DrawQuad;

#[derive(Clone, Default, Debug)]
pub struct DrawScrollBar {
    pub draw_super: DrawQuad,
    pub is_vertical: Option<f32>,
    pub norm_handle: Option<f32>,
    pub norm_scroll: Option<f32>,
}

impl DrawScrollBar {
    pub fn is_vertical(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.is_vertical.replace(f);
        })
    }
    pub fn norm_handle(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.norm_handle.replace(f);
        })
    }
    pub fn norm_scroll(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.norm_scroll.replace(f);
        })
    }
}

impl Display for DrawScrollBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_bar = String::new();
        draw_bar.push_str(self.draw_super.to_string().as_str());

        if let Some(is_vertical) = self.is_vertical.as_ref() {
            draw_bar.push_str(&format!("is_vertical: {}, ", is_vertical));
        }
        if let Some(norm_handle) = self.norm_handle.as_ref() {
            draw_bar.push_str(&format!("norm_handle: {}, ", norm_handle));
        }
        if let Some(norm_scroll) = self.norm_scroll.as_ref() {
            draw_bar.push_str(&format!("norm_scroll: {}, ", norm_scroll));
        }
        write!(f, "{}", draw_bar)
    }
}
