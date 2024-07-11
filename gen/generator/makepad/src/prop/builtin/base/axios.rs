#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::{
    prop::{HORIZONTAL, VERTICAL},
    str_to_string_try_from,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

impl TryFrom<&str> for Axis {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HORIZONTAL => Ok(Axis::Horizontal),
            VERTICAL => Ok(Axis::Vertical),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::Axis!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Axis}

impl TryFrom<&Value> for Axis {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{:?} cannot be converted to Makepad::Axis!",
                        value
                    )))
                })
        }
    }
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Axis::Horizontal => f.write_str(HORIZONTAL),
            Axis::Vertical => f.write_str(VERTICAL),
        }
    }
}
