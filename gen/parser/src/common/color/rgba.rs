use std::fmt::Display;

use crate::Function;

use super::{trans_color_rgb, trans_opacity, Rgb};
use gen_utils::error::Errors;
/// 语法: `rgba(r, g, b, a)`
#[derive(Debug, Clone, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl TryFrom<&Function> for Rgba {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        if value.get_name().eq("rgba") {
            // 检查是否是三个参数，并且都是数字
            if let Some(params) = value.get_params() {
                if params.len() == 4 {
                    // 将三个参数转换为数字且保证在0-255之间
                    let r = trans_color_rgb(&params[0])?;
                    let g = trans_color_rgb(&params[1])?;
                    let b = trans_color_rgb(&params[2])?;
                    let a = trans_opacity(&params[3])?;
                    return Ok(Rgba { r, g, b, a });
                }
            }
            return Err(Errors::ParseError(format!(
                "parse rgb error: {}, rgba fn need four params `(r, g, b, a)`",
                value.get_name()
            )));
        }
        return Err(Errors::ParseError(format!(
            "parse rgba error: {}",
            value.get_name()
        )));
    }
}

impl From<&Rgb> for Rgba {
    fn from(value: &Rgb) -> Self {
        Rgba {
            r: value.r,
            g: value.g,
            b: value.b,
            a: 1.0,
        }
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
