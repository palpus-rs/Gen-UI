use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_visible(value: &Value) -> Result<PropRole, Errors> {
    prop_common_bool("visible", value)
}

pub fn prop_grab_key_focus(value: &Value) -> Result<PropRole, Errors> {
    prop_common_bool("grab_key_focus", value)
}

pub fn prop_block_signal_event(value: &Value) -> Result<PropRole, Errors> {
    prop_common_bool("block_signal_event", value)
}

pub fn prop_common_bool(k: &str, value: &Value) -> Result<PropRole, Errors> {
    // Unknown -> String
    match value.is_unknown_and_get() {
        Some(s) => match s.parse::<bool>() {
            Ok(b) => Ok(PropRole::normal(k, MakepadPropValue::Bool(b))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to {}",
                value, k
            ))),
        },
        None => Err(Errors::KnownPropType),
    }
}
