use std::fmt::Display;

use syn::parse::Parse;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DOWN, OVERLAY, RIGHT, RIGHTWRAP},
};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    /// default
    Right,
    Down,
    Overlay,
    RightWrap,
}

impl MapValue for Flow {
    fn map_value_code(&self) -> String {
        match self {
            Flow::Right => "Flow::Right".to_string(),
            Flow::Down => "Flow::Down".to_string(),
            Flow::Overlay => "Flow::Overlay".to_string(),
            Flow::RightWrap => "Flow::RightWrap".to_string(),
        }
    }
}

impl TryFrom<&str> for Flow {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            RIGHT => Ok(Flow::Right),
            DOWN => Ok(Flow::Down),
            OVERLAY => Ok(Flow::Overlay),
            RIGHTWRAP => Ok(Flow::RightWrap),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Flow!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Flow}

impl Display for Flow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flow::Right => f.write_str(RIGHT),
            Flow::Down => f.write_str(DOWN),
            Flow::Overlay => f.write_str(OVERLAY),
            Flow::RightWrap => f.write_str(RIGHTWRAP),
        }
    }
}

impl Parse for Flow {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let ident_str = ident.to_string();
        match ident_str.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("{} cannot be converted to Makepad::Flow!", ident_str),
            )),
        }
    }
}
