use std::fmt::Display;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{ALL, FILL, FIT},
};

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
