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
pub fn prop_margin(value: &Value) -> Result<PropRole,Errors>{
    match value.is_unknown_and_get() {
        Some(s) =>{
            match s.try_into() {
                Ok(margin) => Ok(
                    PropRole::normal("margin",  MakepadPropValue::Margin(margin))
                ),
                Err(e) => Err(e),
            }
        },
        None => Err(Errors::KnownPropType),
    }
}
