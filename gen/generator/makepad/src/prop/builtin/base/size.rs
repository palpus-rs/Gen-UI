use std::default;
use std::{fmt::Display, num::ParseFloatError};

use gen_converter::error::Errors;

use crate::prop::{ALL, FILL, FIT};
use crate::str_to_string_try_from;

#[derive(Debug,Clone,Default)]

/// # Makepad Size
/// the size of props
/// - height
/// - width
pub enum Size {
    #[default]
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
