use std::fmt::Display;

use crate::{prop::builtin::CheckType, widget::utils::f32_prop};
use gen_parser::Value;
use gen_utils::error::Errors;

// use super::draw_quad::DrawQuad;

#[derive(Debug, Clone, Default)]
pub struct DrawCheckBox {
    // pub draw_super: Option<DrawQuad>,
    pub check_type: Option<CheckType>,
    pub hover: Option<f32>,
    pub focus: Option<f32>,
    pub selected: Option<f32>,
}

impl DrawCheckBox {
    pub fn check_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_type.replace(value.try_into()?);
        Ok(())
    }
    pub fn hover(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.hover.replace(f);
        })
    }
    pub fn focus(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.focus.replace(f);
        })
    }
    pub fn selected(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.selected.replace(f);
        })
    }
}

impl Display for DrawCheckBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_check_box = String::new();
        if let Some(check_type) = &self.check_type {
            draw_check_box.push_str(&format!("check_type: {}, ", check_type));
        }
        if let Some(hover) = &self.hover {
            draw_check_box.push_str(&format!("hover: {}, ", hover));
        }
        if let Some(focus) = &self.focus {
            draw_check_box.push_str(&format!("focus: {}, ", focus));
        }
        if let Some(selected) = &self.selected {
            draw_check_box.push_str(&format!("selected: {}, ", selected));
        }
        write!(f, "{}", draw_check_box)
    }
}
