use parser::Value;

use crate::{
    error::Errors,
    targets::makepad::value::{DAlign, MakepadPropValue},
};

use super::PropRole;

pub fn prop_align(value: &Value) -> Result<PropRole, Errors> {
    prop_common_align(value, DAlign::All)
}

#[allow(dead_code)]
pub fn prop_align_x(value: &Value) -> Result<PropRole, Errors> {
    prop_common_align(value, DAlign::X)
}

#[allow(dead_code)]
pub fn prop_align_y(value: &Value) -> Result<PropRole, Errors> {
    prop_common_align(value, DAlign::Y)
}

pub fn prop_common_align(value: &Value, d_align: DAlign) -> Result<PropRole, Errors> {
    let handle = |s, d_align| {
        (s, d_align)
            .try_into()
            .map(|align| PropRole::normal("align", MakepadPropValue::Align(align)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s, d_align)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "align",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s, d_align))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert Align",
                    value
                )))
            })
    }
}
