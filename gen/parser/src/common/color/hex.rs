use std::str::FromStr;
use gen_utils::error::Errors;

use super::{parse_hex_color, Rgb, Rgba};

/// 16进制颜色
#[derive(Debug, Clone)]
pub struct Hex(pub String);

impl FromStr for Hex {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((remain, hex)) = parse_hex_color(s) {
            if remain.is_empty() {
                Ok(Hex(format!("#{}", hex)))
            } else {
                Err(Errors::ParseError(format!(
                    "parse hex color error, remain: {}",
                    remain
                )))
            }
        } else {
            Err(Errors::ParseError("parse hex color error".to_string()))
        }
    }
}

impl From<&Rgb> for Hex {
    fn from(value: &Rgb) -> Self {
        Hex(format!("#{:02x}{:02x}{:02x}", value.r, value.g, value.b))
    }
}

impl From<&Rgba> for Hex {
    fn from(value: &Rgba) -> Self {
        Hex(format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            value.r,
            value.g,
            value.b,
            (value.a * 255.0) as u8
        ))
    }
}
