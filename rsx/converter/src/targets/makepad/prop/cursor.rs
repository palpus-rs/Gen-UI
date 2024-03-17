use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_cursor(value: &Value) -> Result<PropRole, Errors> {
    let handle = |s: &String| {
        s.try_into()
            .map(|cursor| PropRole::normal("cursor", MakepadPropValue::Cursor(cursor)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "cursor",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to Cursor",
                    value
                )))
            })
    }
}
