use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_text(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        Ok(PropRole::normal(
            "text",
            MakepadPropValue::String(s.to_string()),
        ))
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "text",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                Ok(PropRole::normal(
                    "text",
                    MakepadPropValue::String(s.to_string()),
                ))
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to text",
                    value
                )))
            })
    }
}
