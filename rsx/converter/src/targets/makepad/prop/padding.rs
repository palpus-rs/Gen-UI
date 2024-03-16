use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// Convert padding to Makepad Padding
/// ## single
/// - rsx:      `padding: 10`
/// - makepad:  `padding: 10`
/// ### multi 2
/// - rsx:      `padding: 10 20`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 10, left: 20}`
/// ### multi 4
/// - rsx:      `padding: 10 20 0 29`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 0, left: 29}`
pub fn prop_padding(value: &Value) -> Result<PropRole, Errors> {
    let handle = |s: &String| {
        s.try_into()
            .map(|padding| PropRole::normal("padding", MakepadPropValue::Padding(padding)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "padding",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to padding",
                    value
                )))
            })
    }
}
