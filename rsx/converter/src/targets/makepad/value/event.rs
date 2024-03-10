use std::fmt::Display;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DOWN, UP},
};

#[derive(Debug, Clone, PartialEq)]
pub enum EventOrder {
    Down,
    /// default
    Up,
    // List(Vec<>),
}

impl TryFrom<&str> for EventOrder {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            DOWN => Ok(EventOrder::Down),
            UP => Ok(EventOrder::Up),
            _ => Err(Errors::PropConvertFail(format!(
                "{} can not convert to EventOrder",
                value
            ))),
        }
    }
}

str_to_string_try_from! {EventOrder}

impl Display for EventOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventOrder::Down => f.write_str(DOWN),
            EventOrder::Up => f.write_str(UP),
            // EventOrder::List(l) => todo!("EventOrder::List"),
        }
    }
}
