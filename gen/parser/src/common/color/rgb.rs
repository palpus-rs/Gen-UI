use std::str::FromStr;

use super::{trans_color_rgb, Hex, Rgba};
use crate::Function;
use gen_utils::error::Errors;

/// 语法: `rgb(r, g, b)`
#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TryFrom<&Function> for Rgb {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        // 检查是否fn的名称叫rgb
        if value.get_name().eq("rgb") {
            // 检查是否是三个参数，并且都是数字
            if let Some(params) = value.get_params() {
                if params.len() == 3 {
                    // 将三个参数转换为数字且保证在0-255之间
                    let r = trans_color_rgb(&params[0])?;
                    let g = trans_color_rgb(&params[1])?;
                    let b = trans_color_rgb(&params[2])?;
                    return Ok(Rgb { r, g, b });
                }
            }
            return Err(Errors::ParseError(format!(
                "parse rgb error: {}, rgb fn need three params `(r, g, b)`",
                value.get_name()
            )));
        }
        return Err(Errors::ParseError(format!(
            "parse rgb error: {}",
            value.get_name()
        )));
    }
}

impl From<&Rgba> for Rgb {
    fn from(value: &Rgba) -> Self {
        Rgb {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<&Hex> for Rgb {
    fn from(value: &Hex) -> Self {
        let hex = value.0.trim_start_matches("#");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Rgb { r, g, b }
    }
}

impl FromStr for Rgb {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 对于str来说如果要转为rgb需要先转hex
        let hex = Hex::from_str(s)?;
        Ok(Rgb::from(&hex))
    }
}
