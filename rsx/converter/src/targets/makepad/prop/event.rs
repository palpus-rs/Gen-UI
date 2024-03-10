use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_event_order(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(eo) => Ok(PropRole::normal(
                "event_order",
                MakepadPropValue::EventOrder(eo),
            )),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
