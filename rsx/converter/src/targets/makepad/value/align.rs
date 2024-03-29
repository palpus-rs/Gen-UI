use std::{fmt::Display, num::ParseFloatError};

use syn::parse::Parse;

use crate::error::Errors;

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum DAlign {
    X,
    Y,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Align {
    pub x: f64,
    pub y: f64,
}

impl MapValue for Align {
    fn map_value_code(&self) -> String {
        format!("Align {{ x:{}, y:{} }}", self.x, self.y)
    }
}

impl Align {
    pub fn new(x: f64, y: f64) -> Align {
        Align { x, y }
    }
    pub fn single_x(x: f64) -> Align {
        Align::new(x, f64::default())
    }
    pub fn single_y(y: f64) -> Align {
        Align::new(f64::default(), y)
    }
}

/// # convert align to Makepad Align
/// ## single
/// - rsx:     `align: 16`
/// - makepad: `align: {x: 16, y:16}`
/// ## multi
/// - rsx:     `align:16 24`
/// - makepad: `align: {x: 16, y:24}`
impl TryFrom<(&str, DAlign)> for Align {
    type Error = Errors;

    fn try_from(value: (&str, DAlign)) -> Result<Self, Self::Error> {
        match value
            .0
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(aligns) => match aligns.len() {
                1 => match value.1 {
                    DAlign::X => Ok(Align::single_x(aligns[0])),
                    DAlign::Y => Ok(Align::single_y(aligns[0])),
                    DAlign::All => Ok(Align::new(aligns[0], aligns[0])),
                },
                2 => Ok(Align::new(aligns[0], aligns[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} cannot be converted to Makepad::Align!",
                    value.0
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Align!",
                value.0
            ))),
        }
    }
}

impl TryFrom<(&String, DAlign)> for Align {
    type Error = Errors;

    fn try_from(value: (&String, DAlign)) -> Result<Self, Self::Error> {
        (value.0.as_str(), value.1).try_into()
    }
}

impl TryFrom<&str> for Align {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(aligns) => match aligns.len() {
                1 => Ok(Align::new(aligns[0], aligns[0])),
                2 => Ok(Align::new(aligns[0], aligns[1])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} cannot be converted to Makepad::Align!",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Align!",
                value
            ))),
        }
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{x: {}, y: {}}}", self.x, self.y))
    }
}

impl Parse for Align {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let value = ident.to_string();

        match value.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("value: {} can not convert to Makepad Align", value),
            )),
        }
    }
}
