use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_visible(value: &Value) -> Result<PropRole, Errors> {
    // Unknown -> String
    match value.is_unknown_and_get() {
        Some(s) => match s.parse::<bool>() {
            Ok(visible) => Ok(PropRole::normal("visible", MakepadPropValue::Bool(visible))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to visible",
                value
            ))),
        },
        None => Err(Errors::KnownPropType),
    }
}
