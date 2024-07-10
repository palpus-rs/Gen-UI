use std::fmt::Display;

use crate::{prop::builtin::DesktopButtonType, widget::utils::f32_prop};
use gen_parser::Value;
use gen_utils::error::Errors;

use super::draw_quad::DrawQuad;

#[derive(Debug, Clone, Default)]
pub struct DrawDesktopButton {
    pub draw_super: DrawQuad,
    pub button_type: Option<DesktopButtonType>,
    pub hover: Option<f32>,
    pub pressed: Option<f32>,
}

impl DrawDesktopButton {
    pub fn button_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.button_type.replace(value.try_into()?);
        Ok(())
    }
    pub fn hover(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.hover.replace(f);
        })
    }

    pub fn pressed(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.pressed.replace(f);
        })
    }
    pub fn color(&mut self, value: &Value) -> Result<(), Errors> {
        let quad = DrawQuad::try_from(value)?;
        self.draw_super = quad;
        Ok(())
    }
}

impl Display for DrawDesktopButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_check_box = String::new();
        draw_check_box.push_str(self.draw_super.to_string().as_str());

        if let Some(button_type) = self.button_type.as_ref() {
            draw_check_box.push_str(&format!("button_type: {}, ", button_type));
        }
        if let Some(hover) = self.hover.as_ref() {
            draw_check_box.push_str(&format!("hover: {}, ", hover));
        }
        if let Some(pressed) = self.pressed.as_ref() {
            draw_check_box.push_str(&format!("pressed: {}, ", pressed));
        }

        write!(f, "{}", draw_check_box)
    }
}
