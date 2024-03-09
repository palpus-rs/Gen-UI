use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_spacing(value: &Value) -> Result<PropRole,Errors>{
    prop_common_spacing("spacing",value)
}

pub fn prop_line_spacing(value: &Value) -> Result<PropRole,Errors>{
    prop_common_spacing("line_spacing",value)
}

pub fn prop_common_spacing(ty:&str,value: &Value) -> Result<PropRole,Errors>{
    match value.is_unknown_and_get() {
        Some(s) =>{
            match s.parse::<f64>() {
                Ok(spacing) => Ok(
                    PropRole::normal(ty,  MakepadPropValue::F64(spacing))
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