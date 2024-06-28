#[allow(unused_imports)]
use std::default;
use std::fmt::Display;

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::prop::enum_ident::FIT;
use crate::prop::{ALL, FILL};
use crate::str_to_string_try_from;

#[derive(Debug, Clone, Default)]

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

impl TryFrom<&Value> for Size {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else if let Some(d) = value.is_double_and_get() {
            Ok(d.into())
        } else if let Some(d) = value.is_float_and_get() {
            Ok((d as f64).into())
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to Size",
                        value
                    )))
                })
        }
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
