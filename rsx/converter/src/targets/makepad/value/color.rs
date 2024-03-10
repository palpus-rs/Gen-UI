use std::fmt::Display;

use parser::common::parse_hex_color;

use crate::{error::Errors, str_to_string_try_from};

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Normal(String),
    LinearGradient,
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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Normal(color) => f.write_fmt(format_args!("#{}", color)),
            Color::LinearGradient => todo!("wait to handle color linear gradient"),
        }
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
