use parser::Value;

use crate::{error::Errors, targets::makepad::{value::MakepadPropValue, Widgets}};

use super::PropRole;

pub fn prop_inherits(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        Ok(PropRole::Component(s.into()))
    } else {
        if let Some(s) = value.is_string_and_get() {
           Ok(PropRole::Component(s.into()))
        }else{
            Err(Errors::PropConvertFail(format!(
                "{} can not convert to inherits",
                value
            )))
        }
    }
    
}

pub fn prop_props(value: &Value) -> Result<PropRole, Errors> {
    match value {
        Value::Bind(b) => Ok(PropRole::bind("$props", MakepadPropValue::bind_without_value(b))),
        _=>panic!(r#"component props can only be bind `:props="component_props"`"#)
    }
}

pub fn action_actions(value: &Value) -> Result<PropRole, Errors> {
    dbg!(value);
    todo!()
}