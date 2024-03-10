use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_cursor(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(cursor) => Ok(PropRole::normal("cursor", MakepadPropValue::Cursor(cursor))),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
