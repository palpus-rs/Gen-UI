use parser::Value;
use syn::token::In;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// Convert Prop `height="190"` to Makepad Prop Size
/// - `height: 190`
/// - `height: Full`
/// - `height: Fit`
/// - `height: All`
pub fn prop_height(value: &Value) -> Result<PropRole, Errors> {
    prop_size("height", value)
}
pub fn prop_width(value: &Value) -> Result<PropRole, Errors> {
    prop_size("width", value)
}

/// Convert to Makepad unified Size
pub fn prop_size(key: &str, value: &Value) -> Result<PropRole, Errors> {
    let handle = |s: &String| {
        s.try_into()
            .map(|size| PropRole::normal(key, MakepadPropValue::Size(size)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(key, MakepadPropValue::bind_without_value(b)))
    } else if let Some(s) = value.is_string_and_get() {
        handle(s)
    } else {
        match value.is_double_and_get() {
            Some(f) => Ok(PropRole::normal(key, MakepadPropValue::Size(f.into()))),
            None => Err(Errors::UnAcceptConvertRange),
        }
    }

    // match value.is_unknown_and_get() {
    //     Some(s) =>{
    //         match s.try_into() {
    //             Ok(size) => Ok(
    //                 PropRole::Normal(key.to_string(), MakepadPropValue::Size(size))
    //             ),
    //             Err(e) => Err(e),
    //         }
    //     },
    //     None => Err(Errors::KnownPropType),
    // }
}
