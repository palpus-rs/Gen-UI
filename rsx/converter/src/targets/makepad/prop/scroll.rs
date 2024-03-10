use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_scroll(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(scroll) => Ok(PropRole::normal("scroll", MakepadPropValue::DVec2(scroll))),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
