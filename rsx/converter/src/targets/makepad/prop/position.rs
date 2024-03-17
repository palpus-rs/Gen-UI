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
    let handle = |s: &String| {
        s.try_into()
            .map(|abs_pos| PropRole::normal("abs_pos", MakepadPropValue::DVec2(abs_pos)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "abs_pos",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to abs_pos",
                    value
                )))
            })
    }
}
