use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;
use proc_macro2::TokenStream;

use crate::{widget::utils::f32_prop, ToToken};

use super::draw_text::DrawText;

#[derive(Debug, Clone, Default)]
pub struct DrawLabelText {
    pub draw_super: DrawText,
    pub focus: Option<f32>,
    pub hover: Option<f32>,
    pub pressed: Option<f32>,
}

impl DrawLabelText {
    pub fn focus(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.focus.replace(f);
        })
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
}


impl Display for DrawLabelText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_label = String::new();
        draw_label.push_str(self.draw_super.to_string().as_str());

        if let Some(focus) = self.focus.as_ref() {
            draw_label.push_str(&format!("focus: {}, ", focus));
        }
        if let Some(hover) = self.hover.as_ref() {
            draw_label.push_str(&format!("hover: {}, ", hover));
        }
        if let Some(pressed) = self.pressed.as_ref() {
            draw_label.push_str(&format!("pressed: {}, ", pressed));
        }

        f.write_str(draw_label.as_str())
    }
}

impl ToToken for DrawLabelText {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}
