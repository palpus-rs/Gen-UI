use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// Convert to Makepad Walk abs_pos
/// ## single
/// - rsx:      `absolute_position: 12;`
/// - makepad:  `abs_pos: vec2(12, 12)`
/// ## multi
/// - rsx:      `absolute_position: 12 20;`
/// - makepad:  `abs_pos: vec2(12, 20)`
pub fn prop_abs_prop(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(abs_pos) => Ok(PropRole::normal(
                "abs_pos",
                MakepadPropValue::DVec2(abs_pos),
            )),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
