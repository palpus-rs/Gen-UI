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
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(padding) => Ok(PropRole::normal(
                "padding",
                MakepadPropValue::Padding(padding),
            )),
            Err(e) => Err(e),
        },
        None => match value.is_bind_and_get() {
            Some(b) => Ok(PropRole::bind(
                "padding",
                MakepadPropValue::bind_without_value(b),
            )),
            None => todo!(),
        },
    }
}
