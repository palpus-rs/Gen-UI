use std::fmt::Display;

use gen_parser::{common::parse_hex_color, Value};
use gen_utils::error::Errors;
use syn::parse::Parse;

use crate::str_to_string_try_from;

use super::draw_quad::DrawQuad;

// use super::MapValue;

#[derive(Debug, Clone, Default)]
pub struct DrawColor {
    pub color: Option<String>,
    pub draw_super: DrawQuad,
}

impl TryFrom<&Value> for DrawColor {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let (quad, hex_color) = DrawQuad::try_from_back(value)?;
        let mut draw_color = DrawColor::default();
        // exist color
        draw_color.color = hex_color.map(|hex| hex.0);
        draw_color.draw_super = quad;

        Ok(draw_color)
    }
}

impl Display for DrawColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(color) = &self.color {
            f.write_fmt(format_args!("color: {}", color))
        } else {
            self.draw_super.fmt(f)
        }
    }
}

// impl Default for DrawColor {
//     fn default() -> Self {
//         DrawColor::Color("#FFFFFF00".to_string())
//     }
// }

// impl DrawColor {
//     pub fn is_font(&self) -> bool {
//         matches!(self, DrawColor::Color(_))
//     }
// }

// impl TryFrom<&str> for DrawColor {
//     type Error = Errors;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match parse_hex_color(value) {
//             Ok((input, color)) => {
//                 if input.is_empty() {
//                     return Ok(DrawColor::DrawColor(color));
//                 }
//                 Err(Errors::PropConvertFail(format!(
//                     "{} is not a right hex color",
//                     value
//                 )))
//             }
//             Err(_) => Err(Errors::PropConvertFail(format!(
//                 "{} is not a right hex color",
//                 value
//             ))),
//         }
//     }
// }

// str_to_string_try_from! {DrawColor}

// /// for font
// /// - bool : true for font
// /// - bool : false for normal color
// impl TryFrom<(&str, bool)> for DrawColor {
//     type Error = Errors;

//     fn try_from(value: (&str, bool)) -> Result<Self, Self::Error> {
//         match parse_hex_color(value.0) {
//             Ok((input, color)) => {
//                 if input.is_empty() {
//                     if value.1 {
//                         return Ok(DrawColor::Color(color));
//                     }
//                     return Ok(DrawColor::DrawColor(color));
//                 }
//                 Err(Errors::PropConvertFail(format!(
//                     "{} is not a right hex color",
//                     value.0
//                 )))
//             }
//             Err(_) => Err(Errors::PropConvertFail(format!(
//                 "{} is not a right hex color",
//                 value.0
//             ))),
//         }
//     }
// }

// impl TryFrom<(&String, bool)> for DrawColor {
//     type Error = Errors;

//     fn try_from(value: (&String, bool)) -> Result<Self, Self::Error> {
//         (value.0.as_str(), value.1).try_into()
//     }
// }

// impl TryFrom<(&Value, bool)> for DrawColor {
//     type Error = Errors;

//     fn try_from(value: (&Value, bool)) -> Result<Self, Self::Error> {
//         if let Some(s) = value.0.is_unknown_and_get() {
//             (s, value.1).try_into()
//         } else {
//             value
//                 .0
//                 .is_string_and_get()
//                 .map(|s| (s, value.1).try_into())
//                 .unwrap_or_else(|| {
//                     Err(Errors::PropConvertFail(format!(
//                         "{} can not convert to DrawColor",
//                         value.0
//                     )))
//                 })
//         }
//     }
// }

// impl Display for DrawColor {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             DrawColor::DrawColor(color) => f.write_fmt(format_args!("#{}", color)),
//             DrawColor::Color(font) => f.write_fmt(format_args!("#{}", font)),
//             DrawColor::LinearGradient => todo!("wait to handle color linear gradient"),
//         }
//     }
// }

// impl Parse for DrawColor {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let ident = input.parse::<syn::Ident>()?;
//         let ident_str = ident.to_string();
//         dbg!(ident_str);
//         todo!("Color parse  waiting to impl syn::parse::Parse")
//     }
// }
