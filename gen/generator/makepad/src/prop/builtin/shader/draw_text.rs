use std::fmt::Display;

use gen_parser::{
    common::{hex_to_vec4, BuiltinColor, Hex},
    Value,
};
use gen_utils::error::Errors;
use proc_macro2::TokenStream;

use crate::{
    prop::{
        builtin::{
            
            utils::{draw_linear_gradient, draw_radial_gradient, get_color}, Font, TextWrap
        },
        BRIGHTNESS, COLOR, COMBINE_SPACES, CURVE, DRAW_DEPTH, FONT, FONT_SCALE, FONT_SIZE,
        HEIGHT_FACTOR, INGORE_NEWLINES, LINE_SPACING, TEXT_STYLE, TOP_DROP, WRAP,
    },
    widget::utils::{bool_prop, f32_prop, f64_prop},
    ToToken,
};


#[derive(Clone, Default, Debug)]
pub struct DrawText {
    //pub geometry: GeometryQuad2D,
    pub text_style: Option<TextStyle>,
    pub wrap: Option<TextWrap>,
    pub ignore_newlines: Option<bool>,
    pub combine_spaces: Option<bool>,
    pub font_scale: Option<f64>,
    pub draw_depth: Option<f32>,
    pub color: Option<Hex>,
    pub get_color: TokenStream,
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

        let color = BuiltinColor::try_from(value)?;

        match color {
            BuiltinColor::Hex(hex) => {
                self.get_color = get_color(&hex);
                self.color = Some(hex);
            }
            BuiltinColor::Rgb(rgb) => {
                let hex = Hex::from(&rgb);
                self.get_color = get_color(&hex);
                self.color = Some(hex);
            }
            BuiltinColor::Rgba(rgba) => {
                let hex = Hex::from(&rgba);
                self.get_color = get_color(&hex);
                self.color = Some(hex);
            }
            BuiltinColor::LinearGradient(linear) => {
                self.get_color = draw_linear_gradient(&linear, "get_color");
            }
            BuiltinColor::RadialGradient(radial) => {
                self.get_color = draw_radial_gradient(&radial, "get_color");
            }
            BuiltinColor::Shader(shader) => self.get_color = shader.0,
        }
        Ok(())
    }
}

impl ToToken for DrawText {
    fn to_token_stream(&self) -> TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl Display for DrawText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut draw_text = String::new();
        if let Some(text_style) = &self.text_style {
            draw_text.push_str(&format!("{}: {{{}}},", TEXT_STYLE, text_style));
        }
        if let Some(wrap) = &self.wrap {
            draw_text.push_str(&format!("{}: {},", WRAP, wrap));
        }
        if let Some(ignore_newlines) = &self.ignore_newlines {
            draw_text.push_str(&format!("{}: {},", INGORE_NEWLINES, ignore_newlines));
        }
        if let Some(combine_spaces) = &self.combine_spaces {
            draw_text.push_str(&format!("{}: {},", COMBINE_SPACES, combine_spaces));
        }
        if let Some(font_scale) = &self.font_scale {
            draw_text.push_str(&format!("{}: {},", FONT_SCALE, font_scale));
        }
        if let Some(draw_depth) = &self.draw_depth {
            draw_text.push_str(&format!("{}: {},", DRAW_DEPTH, draw_depth));
        }
        if let Some(color) = &self.color {
            draw_text.push_str(&format!("{}: {},", COLOR, hex_to_vec4(color)));
        } else {
            if !self.get_color.is_empty() {
                draw_text.push_str(self.get_color.to_string().as_str());
            }
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

impl ToToken for TextStyle {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.to_string().parse::<TokenStream>().unwrap()
    }
}

impl Display for TextStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text_style = String::new();
        if let Some(font) = &self.font {
            
            text_style.push_str(&format!("{}: {{{}}},", FONT, font));
        }
        if let Some(font_size) = &self.font_size {
            text_style.push_str(&format!("{}: {},", FONT_SIZE, font_size));
        }
        if let Some(brightness) = &self.brightness {
            text_style.push_str(&format!("{}: {},", BRIGHTNESS, brightness));
        }
        if let Some(curve) = &self.curve {
            text_style.push_str(&format!("{}: {},", CURVE, curve));
        }
        if let Some(line_spacing) = &self.line_spacing {
            text_style.push_str(&format!("{}: {},", LINE_SPACING, line_spacing));
        }
        if let Some(top_drop) = &self.top_drop {
            text_style.push_str(&format!("{}: {},", TOP_DROP, top_drop));
        }
        if let Some(height_factor) = &self.height_factor {
            text_style.push_str(&format!("{}: {},", HEIGHT_FACTOR, height_factor));
        }
        write!(f, "{}", text_style)
    }
}

#[cfg(test)]
mod test_draw_text {
    use super::*;
    #[test]
    fn text_style_to_tk() {
        let mut text_style = TextStyle::default();

        text_style.font = Some(
            "crate://self/resources/icons/Icon_Search.svg"
                .try_into()
                .unwrap(),
        );
        text_style.font_size = Some(12.0_f64.try_into().unwrap());
        text_style.brightness = Some(0.5.try_into().unwrap());
        text_style.curve = Some(0.5.try_into().unwrap());
        text_style.line_spacing = Some(1.5_f64.try_into().unwrap());
        text_style.top_drop = Some(1.0_f64.try_into().unwrap());
        text_style.height_factor = Some(1.0_f64.try_into().unwrap());
        let tk = text_style.to_token_stream();
        let prop = "font : dep (\"crate://self/resources/icons/Icon_Search.svg\") , font_size : 12 , brightness : 0.5 , curve : 0.5 , line_spacing : 1.5 , top_drop : 1 , height_factor : 1 ,";

        assert_eq!(tk.to_string().as_str(), prop);
    }

    #[test]
    fn draw_text_to_tk() {
        let mut draw_text = DrawText::default();
        draw_text.text_style = Some(TextStyle {
            font: Some(
                "crate://self/resources/icons/Icon_Search.svg"
                    .try_into()
                    .unwrap(),
            ),
            font_size: Some(12.0_f64.try_into().unwrap()),
            brightness: Some(0.5.try_into().unwrap()),
            curve: Some(0.5.try_into().unwrap()),
            line_spacing: Some(1.5_f64.try_into().unwrap()),
            top_drop: Some(1.0_f64.try_into().unwrap()),
            height_factor: Some(1.0_f64.try_into().unwrap()),
        });
        draw_text.wrap = Some(TextWrap::Ellipsis);
        draw_text.ignore_newlines = Some(true);
        draw_text.combine_spaces = Some(true);
        draw_text.font_scale = Some(1.0_f64.try_into().unwrap());
        draw_text.draw_depth = Some(0.5_f32.try_into().unwrap());
        draw_text.color = Some("#445566".try_into().unwrap());
        let tk = draw_text.to_token_stream();
        let prop = "text_style : { font : dep (\"crate://self/resources/icons/Icon_Search.svg\") , font_size : 12 , brightness : 0.5 , curve : 0.5 , line_spacing : 1.5 , top_drop : 1 , height_factor : 1 , } , wrap : Ellipsis , ignore_newlines : true , combine_spaces : true , font_scale : 1 , draw_depth : 0.5 , color : # 445566 ,";

        assert_eq!(tk.to_string().as_str(), prop);
    }
}
