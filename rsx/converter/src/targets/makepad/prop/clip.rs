use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_clip_x(value: &Value) -> Result<PropRole, Errors> {
    prop_common_clip("clip_x", value)
}

pub fn prop_clip_y(value: &Value) -> Result<PropRole, Errors> {
    prop_common_clip("clip_y", value)
}

pub fn prop_common_clip(ty: &str, value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        let clip = s.parse::<bool>().unwrap();
        Ok(PropRole::normal(ty, MakepadPropValue::Bool(clip)))
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(ty, MakepadPropValue::bind_without_value(b)))
    } else {
        value
            .is_bool_and_get()
            .map(|clip| Ok(PropRole::normal(ty, MakepadPropValue::Bool(clip))))
            .unwrap_or_else(|| Err(Errors::UnAcceptConvertRange))
    }
}
