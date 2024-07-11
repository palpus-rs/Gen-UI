use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::{prop::{HORIZONTAL, ROTARY, VERTICAL}, str_to_string_try_from};

#[derive(Debug, Clone, Copy, Default)]
pub enum SliderType {
    #[default]
    Horizontal,
    Vertical,
    Rotary
}

impl TryFrom<&str> for SliderType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HORIZONTAL => Ok(SliderType::Horizontal),
            VERTICAL => Ok(SliderType::Vertical),
            ROTARY => Ok(SliderType::Rotary),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::SliderType!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {SliderType}

impl TryFrom<&Value> for SliderType {
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
                        "{:?} cannot be converted to Makepad::SliderType!",
                        value
                    )))
                })
        }
    }
}

impl Display for SliderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliderType::Horizontal => f.write_str(HORIZONTAL),
            SliderType::Vertical => f.write_str(VERTICAL),
            SliderType::Rotary => f.write_str(ROTARY),
        }
    }
}