use std::fmt::Display;

use crate::{
    error::Errors,
    targets::makepad::constants::{ALL, FILL, FIT},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Fill,
    Fixed(f64),
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

impl TryFrom<&String> for Size {
    type Error = Errors;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
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
