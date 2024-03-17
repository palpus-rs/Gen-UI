use std::{fmt::Display, num::ParseFloatError};

use syn::parse::Parse;

use crate::{error::Errors, str_to_string_try_from};

use super::MapValue;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl MapValue for DVec2 {
    fn map_value_code(&self) -> String {
        format!("DVec2 {{ x:{}, y:{} }}", self.x, self.y)
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

impl Display for DVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("vec2({}, {})", self.x, self.y))
    }
}

impl Parse for DVec2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let value = ident.to_string();
        match value.as_str().try_into() {
            Ok(vec2) => Ok(vec2),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("value: {} can not convert to Makepad DVec2", value),
            )),
        }
    }
}
