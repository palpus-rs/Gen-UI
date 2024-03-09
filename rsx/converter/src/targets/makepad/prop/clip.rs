use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_clip_x(value: &Value) -> Result<PropRole,Errors>{
    prop_common_clip("clip_x",value)
}

pub fn prop_clip_y(value: &Value) -> Result<PropRole,Errors>{
    prop_common_clip("clip_y",value)
}

pub fn prop_common_clip(ty:&str,value: &Value) -> Result<PropRole,Errors>{
    match value.is_unknown_and_get() {
        Some(s) =>{
            match s.parse::<bool>() {
                Ok(clip) => Ok(
                    PropRole::normal(ty,  MakepadPropValue::Bool(clip))
                ),
                Err(_) => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to {}",
                    value,ty
                ))),
            }
        },
        None => Err(Errors::KnownPropType),
    }
}