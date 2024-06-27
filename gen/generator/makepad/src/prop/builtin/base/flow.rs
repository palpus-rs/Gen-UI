#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::{
    prop::{DOWN, OVERLAY, RIGHT, RIGHTWRAP},
    str_to_string_try_from,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum Flow {
    #[default]
    Right,
    Down,
    //Left,
    //Up,
    Overlay,
    RightWrap,
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

impl TryFrom<&Value> for Flow {
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
                        "{:?} cannot be converted to Makepad::Flow!",
                        value
                    )))
                })
        }
    }
}

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
