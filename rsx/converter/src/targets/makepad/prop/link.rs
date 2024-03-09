use parser::Value;

use crate::error::Errors;

use super::PropRole;

/// Convert Prop `class="xxx"` to Makepad Prop 
/// - `class="xxx"`
/// - `:class="xxx"` (wait to write)
/// - `@class="xxx"` (wait to write)
pub fn prop_class(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => {
           let classs = s.split(' ').map(|x|x.to_string()).collect::<Vec<String>>();
            Ok(PropRole::Context(classs))
        },
        None => Err(Errors::KnownPropType),
    }
}

/// Convert Prop `id="xxx"` to Makepad Prop 
/// - `id="xxx"`
/// - `:id="xxx"` (wait to write)
/// - `@id="xxx"` (wait to write)
pub fn prop_id(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => Ok(PropRole::Special(s.to_owned())),
        None => Err(Errors::KnownPropType),
    }
}
