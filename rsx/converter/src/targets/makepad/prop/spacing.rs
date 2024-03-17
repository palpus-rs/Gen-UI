use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_spacing(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f64("spacing", value)
}

pub fn prop_line_spacing(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f64("line_spacing", value)
}

pub fn prop_common_f64(ty: &str, value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f64>() {
            Ok(spacing) => Ok(PropRole::normal(ty, MakepadPropValue::F64(spacing))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to {}",
                value, ty
            ))),
        }
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(ty, MakepadPropValue::bind_without_value(b)))
    } else {
        value
            .is_double_and_get()
            .map(|f| Ok(PropRole::normal(ty, MakepadPropValue::F64(f))))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to {}",
                    value, ty
                )))
            })
    }
}

pub fn prop_common_f32(ty: &str, value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f32>() {
            Ok(spacing) => Ok(PropRole::normal(ty, MakepadPropValue::F32(spacing))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to {}",
                value, ty
            ))),
        }
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(ty, MakepadPropValue::bind_without_value(b)))
    } else {
        value
            .is_float_and_get()
            .map(|f| Ok(PropRole::normal(ty, MakepadPropValue::F32(f))))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to {}",
                    value, ty
                )))
            })
    }
}
