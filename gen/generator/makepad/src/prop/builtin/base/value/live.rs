use std::{ fmt::Display, str::FromStr};

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::{
    prop::builtin::{DVec2, Vec3, Vec4},
    utils::{float_to_str, float_to_str_f64},
};

#[derive(Clone, Debug)]
pub enum LiveValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    Vec2(DVec2),
    Vec3(Vec3),
    Vec4(Vec4),
    // Color,
}

impl LiveValue {
    pub fn try_from_value_vec(value: &Value) -> Result<Vec<LiveValue>, Errors> {
        let v = value.to_vec()?;
        let res = v
            .into_iter()
            .map(|v| LiveValue::try_from(&v).unwrap())
            .collect::<Vec<LiveValue>>();
        Ok(res)
    }
    pub fn vec_to_string(values: &Vec<LiveValue>) -> String {
        format!(
            "[{}]",
            values
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}


impl TryFrom<&Value> for LiveValue {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::USize(num) => Ok(LiveValue::U64(*num as u64)),
            Value::ISize(num) => Ok(LiveValue::I64(*num as i64)),
            Value::Float(f_num) => Ok(LiveValue::F32(*f_num)),
            Value::Double(d_num) => Ok(LiveValue::F64(*d_num)),
            Value::Bool(b) => Ok(LiveValue::Bool(*b)),
            Value::String(s) => Ok(LiveValue::String(s.to_string())),
            Value::UnKnown(u) => {
                let v = Value::from_str(u)?;
                LiveValue::try_from(&v)
            }
            _ => Err(Errors::PropConvertFail(format!(
                "{:?} can not convert to LiveValue",
                value
            ))),
        }
    }
}

impl Display for LiveValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                LiveValue::Bool(b) => b.to_string(),
                LiveValue::I64(i) => i.to_string(),
                LiveValue::U64(u) => u.to_string(),
                LiveValue::F32(f) => float_to_str(*f),
                LiveValue::F64(f) => float_to_str_f64(*f),
                LiveValue::String(s) => format!("\"{}\"", s),
                LiveValue::Vec2(v) => v.to_string(),
                LiveValue::Vec3(v) => v.to_string(),
                LiveValue::Vec4(v) => v.to_string(),
            }
            .as_str(),
        )
    }
}

#[derive(Clone, Debug)]
pub enum LiveValueType {
    Bool,
    I64,
    U64,
    F32,
    F64,
    String,
    Vec2,
    Vec3,
    Vec4,
}

impl TryFrom<&str> for LiveValueType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "bool" => Ok(LiveValueType::Bool),
            "i8" | "i16" | "i32" | "i64" | "isize" => Ok(LiveValueType::I64),
            "u8" | "u16" | "u32" | "u64" | "usize" => Ok(LiveValueType::U64),
            "f32" => Ok(LiveValueType::F32),
            "f64" => Ok(LiveValueType::F64),
            "String" => Ok(LiveValueType::String),
            "Vec2" => Ok(LiveValueType::Vec2),
            "Vec3" => Ok(LiveValueType::Vec3),
            "Vec4" => Ok(LiveValueType::Vec4),
            _ => Err(Errors::PropConvertFail(format!(
                "Cannot convert {} to LiveValueType",
                value
            ))),
        }
    }
}
