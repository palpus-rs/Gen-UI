use std::fmt::Display;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DOWN, OVERLAY, RIGHT, RIGHTWRAP},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    /// default
    Right,
    Down,
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
