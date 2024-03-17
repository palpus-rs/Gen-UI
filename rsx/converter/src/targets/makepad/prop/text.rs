use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::{prop_common_f32, prop_common_f64, PropRole};

pub fn prop_text(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        Ok(PropRole::normal(
            "text",
            MakepadPropValue::String(s.to_string()),
        ))
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "text",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                Ok(PropRole::normal(
                    "text",
                    MakepadPropValue::String(s.to_string()),
                ))
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to text",
                    value
                )))
            })
    }
}

pub fn prop_font(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        Ok(PropRole::normal(
            "font",
            MakepadPropValue::Font(s.to_string()),
        ))
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "font",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                Ok(PropRole::normal(
                    "font",
                    MakepadPropValue::Font(s.to_string()),
                ))
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to font",
                    value
                )))
            })
    }
}
pub fn prop_font_size(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f64("font_size", value)
}
pub fn prop_brightness(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f32("brightness", value)
}
// 这个字段可能用于调整字体的某些渲染效果，如字体边缘的曲线度
pub fn prop_curve(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f32("curve", value)
}
pub fn prop_top_drop(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f64("top_drop", value)
}

pub fn prop_height_factor(value: &Value) -> Result<PropRole, Errors> {
    prop_common_f64("height_factor", value)
}
