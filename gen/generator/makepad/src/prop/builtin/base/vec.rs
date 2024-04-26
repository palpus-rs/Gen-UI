use std::fmt::Display;

use std::num::ParseFloatError;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

impl DVec2 {
    pub fn new(x: f64, y: f64) -> DVec2 {
        DVec2 { x, y }
    }
    pub fn single(f: f64) -> DVec2 {
        DVec2::new(f, f)
    }
}

/// Convert to Makepad Walk abs_pos
/// ## single
/// - rsx:      `absolute_position: 12;`
/// - makepad:  `abs_pos: vec2(12, 12)`
/// ## multi
/// - rsx:      `absolute_position: 12 20;`
/// - makepad:  `abs_pos: vec2(12, 20)`
impl TryFrom<&str> for DVec2 {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split_whitespace()
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(DVec2::single(spaces[0])),
                2 => Ok(DVec2::new(spaces[0], spaces[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to DVec2",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to DVec2",
                value
            ))),
        }
    }
}

str_to_string_try_from! {DVec2}

impl TryFrom<&Value> for DVec2{
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get(){
            s.try_into()
        }else {
            Err(Errors::PropConvertFail(format!(
                "{} can not convert to DVec2",
                value
            )))
        }
    }
}

impl Display for DVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vec2({}, {})", self.x, self.y))
    }
}
