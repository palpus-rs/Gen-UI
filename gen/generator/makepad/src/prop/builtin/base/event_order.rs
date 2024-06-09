#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::{
    prop::{DOWN, UP},
    str_to_string_try_from,
};
#[derive(Debug, Clone, PartialEq, Default)]
pub enum EventOrder {
    Down,
    /// default
    #[default]
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

impl TryFrom<&Value> for EventOrder {
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
                        "{} can not convert to EventOrder",
                        value
                    )))
                })
        }
    }
}

impl Display for EventOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventOrder::Down => f.write_str(DOWN),
            EventOrder::Up => f.write_str(UP),
            // EventOrder::List(l) => todo!("EventOrder::List"),
        }
    }
}
