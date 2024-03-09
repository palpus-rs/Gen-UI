use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_text(key: &str, value: &Value) -> Result<PropRole, Errors> {
    // Unknown -> String
    match value.is_unknown_and_get() {
        Some(s) => Ok(PropRole::Normal(
            key.to_string(),
            MakepadPropValue::String(s.to_owned()),
        )),
        None => Err(Errors::KnownPropType),
    }
}


