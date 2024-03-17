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

    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<bool>() {
            Ok(b) => Ok(PropRole::normal(k, MakepadPropValue::Bool(b))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to {}",
                s, k
            ))),
        }
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(k, MakepadPropValue::bind_without_value(b)))
    } else {
        value
            .is_bool_and_get()
            .map(|b| Ok(PropRole::normal(k, MakepadPropValue::Bool(b))))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to {}",
                    value, k
                )))
            })
    }
}
