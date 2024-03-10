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
    match value.is_unknown_and_get() {
        Some(s) => match (s, d_align).try_into() {
            Ok(align) => Ok(PropRole::normal("align", MakepadPropValue::Align(align))),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
