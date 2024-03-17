use std::fmt::Display;

use syn::parse::Parse;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{ALL, FILL, FIT},
};

use super::MapValue;

/// # Makepad Size
/// the size of props
/// - height
/// - width
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    /// Fill the size of the parent widget
    Fill,
    /// detail size of the current widget
    Fixed(f64),
    /// Fit content
    Fit,
    All,
}

impl MapValue for Size {
    fn map_value_code(&self) -> String {
        match self {
            Size::Fill => format!("Size::Fill"),
            Size::Fixed(f) => format!("Size::Fixed({f})"),
            Size::Fit => format!("Size::Fit"),
            Size::All => format!("Size::All"),
        }
    }
}

impl TryFrom<&str> for Size {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<f64>() {
            Ok(number) => Ok(Size::Fixed(number)),
            Err(_) => match value {
                FILL => Ok(Size::Fill),
                ALL => Ok(Size::All),
                FIT => Ok(Size::Fit),
                _ => Err(Errors::PropConvertFail(format!(
                    "value: {} can not convert to Makepad Size",
                    value
                ))),
            },
        }
    }
}

str_to_string_try_from! {Size}

impl From<f64> for Size {
    fn from(value: f64) -> Self {
        Size::Fixed(value)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Fill => f.write_str(FILL),
            Size::Fixed(num) => f.write_str(num.to_string().as_str()),
            Size::Fit => f.write_str(FIT),
            Size::All => f.write_str(ALL),
        }
    }
}

impl Parse for Size {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let value = ident.to_string();
        match value.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("value: {} can not convert to Makepad Size", value),
            )),
        }
    }
}
