use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::{
    prop::builtin::{Font, TextWrap, Vec4},
    widget::utils::{bool_prop, f32_prop, f64_prop},
};

use super::draw_color::DrawColor;

#[derive(Clone, Default, Debug)]
pub struct DrawText {
    //pub geometry: GeometryQuad2D,
    pub text_style: Option<TextStyle>,
    pub wrap: Option<TextWrap>,
    pub ignore_newlines: Option<bool>,
    pub combine_spaces: Option<bool>,
    pub font_scale: Option<f64>,
    pub draw_depth: Option<f32>,
    pub color: Option<DrawColor>,
}

impl DrawText {
    fn check_text_style(&mut self) -> &mut TextStyle {
        if self.text_style.is_none() {
            self.text_style = Some(TextStyle::default());
        }
        self.text_style.as_mut().unwrap()
    }
    pub fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().font(value)
    }
    pub fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().font_size(value)
    }
    pub fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().brightness(value)
    }
    pub fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().curve(value)
    }
    pub fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().line_spacing(value)
    }
    pub fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().top_drop(value)
    }
    pub fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        self.check_text_style().height_factor(value)
    }
    pub fn wrap(&mut self, value: &Value) -> Result<(), Errors> {
        self.wrap = Some(TextWrap::try_from(value)?);
        Ok(())
    }
    pub fn ignore_newlines(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.ignore_newlines = Some(b);
        })
    }
    pub fn combine_spaces(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b| {
            self.combine_spaces = Some(b);
        })
    }
    pub fn font_scale(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.font_scale = Some(f);
        })
    }
    pub fn draw_depth(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.draw_depth = Some(f);
        })
    }
    pub fn color(&mut self, value: &Value) -> Result<(), Errors> {
        self.color = Some((value, true).try_into()?);
        Ok(())
    }
}

impl Display for DrawText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_text = String::new();
        if let Some(text_style) = &self.text_style {
            draw_text.push_str(&format!("text_style: {}, ", text_style.to_string()));
        }
        if let Some(wrap) = &self.wrap {
            draw_text.push_str(&format!("wrap: {}, ", wrap.to_string()));
        }
        if let Some(ignore_newlines) = &self.ignore_newlines {
            draw_text.push_str(&format!("ignore_newlines: {}, ", ignore_newlines));
        }
        if let Some(combine_spaces) = &self.combine_spaces {
            draw_text.push_str(&format!("combine_spaces: {}, ", combine_spaces));
        }
        if let Some(font_scale) = &self.font_scale {
            draw_text.push_str(&format!("font_scale: {}, ", font_scale));
        }
        if let Some(draw_depth) = &self.draw_depth {
            draw_text.push_str(&format!("draw_depth: {}, ", draw_depth));
        }
        if let Some(color) = &self.color {
            draw_text.push_str(&format!("color: {}, ", color.to_string()));
        }
        write!(f, "{}", draw_text)
    }
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

impl TextStyle {
    pub fn font(&mut self, value: &Value) -> Result<(), Errors> {
        self.font = Some(Font::try_from(value)?);
        Ok(())
    }
    pub fn font_size(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.font_size = Some(f);
        })
    }
    pub fn brightness(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.brightness = Some(f);
        })
    }
    pub fn curve(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.curve = Some(f);
        })
    }
    pub fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.line_spacing = Some(f);
        })
    }
    pub fn top_drop(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.top_drop = Some(f);
        })
    }
    pub fn height_factor(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f| {
            self.height_factor = Some(f);
        })
    }
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text_style = String::new();
        if let Some(font) = &self.font {
            text_style.push_str(&format!("font: {}, ", font.to_string()));
        }
        if let Some(font_size) = &self.font_size {
            text_style.push_str(&format!("font_size: {}, ", font_size));
        }
        if let Some(brightness) = &self.brightness {
            text_style.push_str(&format!("brightness: {}, ", brightness));
        }
        if let Some(curve) = &self.curve {
            text_style.push_str(&format!("curve: {}, ", curve));
        }
        if let Some(line_spacing) = &self.line_spacing {
            text_style.push_str(&format!("line_spacing: {}, ", line_spacing));
        }
        if let Some(top_drop) = &self.top_drop {
            text_style.push_str(&format!("top_drop: {}, ", top_drop));
        }
        if let Some(height_factor) = &self.height_factor {
            text_style.push_str(&format!("height_factor: {}, ", height_factor));
        }
        write!(f, "{}", text_style)
    }
}
