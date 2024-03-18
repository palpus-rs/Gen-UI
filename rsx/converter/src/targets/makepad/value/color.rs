use std::fmt::Display;

use parser::common::parse_hex_color;
use syn::parse::Parse;

use crate::{error::Errors, str_to_string_try_from};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Normal(String),
    Font(String),
    LinearGradient,
}

impl Color {
    pub fn is_font(&self) -> bool {
        matches!(self, Color::Font(_))
    }
}

impl MapValue for Color {
    fn map_value_code(&self) -> String {
        match self {
            Color::Normal(n) => n.to_string(),
            Color::Font(f) => f.to_string(),
            Color::LinearGradient => todo!(),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_hex_color(value) {
            Ok((input, color)) => {
                if input.is_empty() {
                    return Ok(Color::Normal(color));
                }
                Err(Errors::PropConvertFail(format!(
                    "{} is not a right hex color",
                    value
                )))
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} is not a right hex color",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Color}

/// for font
/// - bool : true for font
/// - bool : false for normal color
impl TryFrom<(&str, bool)> for Color {
    type Error = Errors;

    fn try_from(value: (&str, bool)) -> Result<Self, Self::Error> {
        match parse_hex_color(value.0) {
            Ok((input, color)) => {
                if input.is_empty() {
                    if value.1 {
                        return Ok(Color::Font(color));
                    }
                    return Ok(Color::Normal(color));
                }
                Err(Errors::PropConvertFail(format!(
                    "{} is not a right hex color",
                    value.0
                )))
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} is not a right hex color",
                value.0
            ))),
        }
    }
}

impl TryFrom<(&String, bool)> for Color {
    type Error = Errors;

    fn try_from(value: (&String, bool)) -> Result<Self, Self::Error> {
        (value.0.as_str(), value.1).try_into()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Normal(color) => f.write_fmt(format_args!("#{}", color)),
            Color::Font(font) => f.write_fmt(format_args!("#{}", font)),
            Color::LinearGradient => todo!("wait to handle color linear gradient"),
        }
    }
}

impl Parse for Color {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let ident_str = ident.to_string();
        dbg!(ident_str);
        todo!("Color parse  waiting to impl syn::parse::Parse")
    }
}

#[cfg(test)]
mod test_color {
    use super::Color;

    #[test]
    fn test_hex() {
        let colors = vec!["#0", "#f04", "#0388aa"];
        for color in colors {
            dbg!(Color::try_from(color).unwrap().to_string());
        }
    }
}
