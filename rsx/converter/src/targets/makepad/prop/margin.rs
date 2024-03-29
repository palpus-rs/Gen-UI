use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// Convert margin to Makepad Margin
/// ## single
/// - rsx:      `margin: 10`
/// - makepad:  `margin: 10`
/// ### multi 2
/// - rsx:      `margin: 10 20`
/// - makepad:  `margin: {top: 10, right: 20, bottom: 10, left: 20}`
/// ### multi 4
/// - rsx:      `margin: 10 20 0 29`
/// - makepad:  `margin: {top: 10, right: 20, bottom: 0, left: 29}`
pub fn prop_margin(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        s.try_into()
            .map(|margin| PropRole::normal("margin", MakepadPropValue::Margin(margin)))
            .map_err(Into::into)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "margin",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                s.try_into()
                    .map(|margin| PropRole::normal("margin", MakepadPropValue::Margin(margin)))
                    .map_err(Into::into)
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert Margin",
                    value
                )))
            })
    }
}
