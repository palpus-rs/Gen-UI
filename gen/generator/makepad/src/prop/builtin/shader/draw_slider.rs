use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::{prop::builtin::SliderType, widget::utils::f32_prop};

use super::draw_quad::DrawQuad;

#[derive(Debug, Clone, Default)]
pub struct DrawSlider {
    pub draw_super: DrawQuad,
    pub slide_pos: Option<f32>,
    pub slider_type: Option<SliderType>,
}

impl DrawSlider {
    pub fn slider_pos(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.slide_pos = Some(f);
        })
    }
    pub fn slider_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.slider_type = Some(value.try_into()?);
        Ok(())
    }
}

impl Display for DrawSlider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(slide_pos) = self.slide_pos.as_ref() {
            let _ = f.write_fmt(format_args!("slide_pos: {}", slide_pos));
        }
        if let Some(slider_type) = self.slider_type.as_ref() {
            let _ = f.write_fmt(format_args!("slider_type: {}", slider_type));
        }

        f.write_str(&self.draw_super.to_string())
    }
}
