use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// Convert Prop `height="190"` to Makepad Prop Size
/// - `height: 190`
/// - `height: Full`
/// - `height: Fit`
/// - `height: All`
pub fn prop_height(key: &str,value: &Value) -> Result<PropRole,Errors>{
    prop_size(key, value)
}
pub fn prop_width(key: &str,value: &Value)-> Result<PropRole,Errors> {
    prop_size(key, value)
}

/// Convert to Makepad unified Size 
pub fn prop_size(key: &str,value: &Value) -> Result<PropRole,Errors>{
    match value.is_unknown_and_get() {
        Some(s) =>{
            match s.try_into() {
                Ok(size) => Ok(
                    PropRole::Normal(key.to_string(), MakepadPropValue::Size(size))
                ),
                Err(e) => Err(e),
            }
        },
        None => Err(Errors::KnownPropType),
    }
}
