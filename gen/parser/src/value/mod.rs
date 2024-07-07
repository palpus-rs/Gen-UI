mod function;

use std::{fmt::Display, str::FromStr};

pub use function::Function;
use gen_utils::error::Errors;

use crate::{common::BuiltinColor, from_i_number, from_u_number};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    // u type number
    // U8(u8),
    // U16(u16),
    // U32(u32),
    // U64(u64),
    USize(usize),
    // i type number
    // I8(i8),
    // I16(i16),
    // I32(i32),
    // I64(i64),
    ISize(isize),
    // float
    Float(f32),
    Double(f64),
    /// bool
    Bool(bool),
    Vec(Vec<Value>),
    /// String
    String(String),
    /// value inject
    /// <xxx :value="xValue" />
    /// <script> let xValue:&str = "hello!";</script>
    /// <script> let xValue:Vec<&str> = vec!["a","b"];</script>
    Bind(String),
    /// function inject
    /// <xxx @click="doClick" />
    Function(Function),
    /// function return ()  as :`fn xxx()->(){}`
    Void,
    // /// color value
    // /// - hex color: #fff00f
    // /// - rgb color: rgb(211,23,255)
    // /// - rgba color: rgba(255,255,87,0.4)
    // Color(BuiltinColor),
    Struct(String),
    UnKnown(String),
    Dep(String),
}

impl Value {
    pub fn bind(variable: &str) -> Self {
        Self::Bind(variable.to_string())
    }
    pub fn void() -> Self {
        Self::Void
    }
    pub fn is_unknown_and_get(&self) -> Option<&String> {
        match self {
            Value::UnKnown(s) => Some(s),
            _ => None,
        }
    }
    pub fn is_u_int_and_get(&self) -> Option<usize> {
        match self {
            Value::USize(n) => Some(*n),
            _ => None,
        }
    }
    pub fn is_i_int_and_get(&self) -> Option<isize> {
        match self {
            Value::ISize(n) => Some(*n),
            _ => None,
        }
    }
    pub fn is_int_and_get(&self) -> Option<i64> {
        match self {
            // Value::U8(n) => Some(*n as i64),
            // Value::U16(n) => Some(*n as i64),
            // Value::U32(n) => Some(*n as i64),
            // Value::U64(n) => Some(*n as i64),
            Value::USize(n) => Some(*n as i64),
            // Value::I8(n) => Some(*n as i64),
            // Value::I16(n) => Some(*n as i64),
            // Value::I32(n) => Some(*n as i64),
            // Value::I64(n) => Some(*n),
            Value::ISize(n) => Some(*n as i64),
            _ => None,
        }
    }
    pub fn is_bind_and_get(&self) -> Option<&String> {
        match self {
            Value::Bind(b) => Some(b),
            _ => None,
        }
    }
    pub fn is_fn_and_get(&self) -> Option<&Function> {
        match self {
            Value::Function(f) => Some(f),
            _ => None,
        }
    }
    pub fn is_string_and_get(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn is_double_and_get(&self) -> Option<f64> {
        match self {
            Value::Double(d) => Some(*d),
            _ => None,
        }
    }
    pub fn is_float_and_get(&self) -> Option<f32> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn is_bool_and_get(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn is_color_and_get(&self) -> Result<BuiltinColor, Errors> {
        self.try_into()
    }
    pub fn to_vec(&self) -> Result<Vec<Value>, Errors> {
        match self {
            Value::UnKnown(u) => match serde_json::from_str::<serde_json::Value>(u) {
                Ok(v) => {
                    if v.is_array() {
                        let value: Value = v.try_into().unwrap();
                        if let Value::Vec(v) = value {
                            return Ok(v);
                        }
                    }
                    return Err(Errors::PropConvertFail(
                        "value type unsupport to use to_vec()".to_string(),
                    ));
                }
                Err(_) => Err(Errors::PropConvertFail(
                    "can not convert unknown value to Vec<Value>".to_string(),
                )),
            },
            _ => Err(Errors::PropConvertFail(
                "value type unsupport to use to_vec()".to_string(),
            )),
        }
    }
    pub fn to_vec_string(&self) -> Result<Vec<String>, Errors> {
        match self {
            Value::UnKnown(u) => match serde_json::from_str::<serde_json::Value>(u) {
                Ok(v) => {
                    if v.is_array() {
                        let res = v
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_str().unwrap().to_string())
                            .collect::<Vec<String>>();
                        return Ok(res);
                    }
                    return Err(Errors::PropConvertFail(
                        "value type unsupport to use to_vec()".to_string(),
                    ));
                }
                Err(_) => Err(Errors::PropConvertFail(
                    "can not convert unknown value to Vec<Value>".to_string(),
                )),
            },
            _ => Err(Errors::PropConvertFail(
                "value type unsupport to use to_vec_string()".to_string(),
            )),
        }
    }
}

impl From<(&str, Option<Vec<&str>>, bool)> for Value {
    fn from(value: (&str, Option<Vec<&str>>, bool)) -> Self {
        Value::Function(value.into())
    }
}

from_u_number!(u8);
from_u_number!(u16);
from_u_number!(u32);
from_u_number!(u64);
from_i_number!(i8);
from_u_number!(i16);
from_u_number!(i32);
from_u_number!(i64);

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Self {
        Value::String(value.to_string())
    }
}

// impl From<Color> for Value {
//     fn from(value: Color) -> Self {
//         Value::Color(value)
//     }
// }

impl From<Function> for Value {
    fn from(value: Function) -> Self {
        Value::Function(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl TryFrom<serde_json::Value> for Value {
    type Error = Errors;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::Null => Err(Errors::PropConvertFail(
                "null value now is not supported".to_string(),
            )),
            serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
            serde_json::Value::Number(n) => {
                if n.is_f64() {
                    Ok(Value::Double(n.as_f64().unwrap()))
                } else if n.is_i64() {
                    Ok(Value::ISize(n.as_i64().unwrap() as isize))
                } else if n.is_u64() {
                    Ok(Value::USize(n.as_u64().unwrap() as usize))
                } else {
                    Err(Errors::PropConvertFail(
                        "can not convert number to Value::Double | Value::ISize | Value::USize"
                            .to_string(),
                    ))
                }
            }
            serde_json::Value::String(s) => Ok(Value::String(s)),
            serde_json::Value::Array(arr) => {
                // 递归转换
                Ok(Value::Vec(
                    arr.into_iter()
                        .map(|x| x.try_into().unwrap())
                        .collect::<Vec<Value>>(),
                ))
            }
            serde_json::Value::Object(_) => Err(Errors::PropConvertFail(
                "object value now is not supported".to_string(),
            )),
        }
    }
}

impl FromStr for Value {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<serde_json::Value>(s) {
            Ok(v) => v.try_into(),
            Err(e) => Err(Errors::PropConvertFail(e.to_string())),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            // Value::U8(n) => n.to_string(),
            // Value::U16(n) => n.to_string(),
            // Value::U32(n) => n.to_string(),
            // Value::U64(n) => n.to_string(),
            Value::USize(n) => n.to_string(),
            // Value::I8(n) => n.to_string(),
            // Value::I16(n) => n.to_string(),
            // Value::I32(n) => n.to_string(),
            // Value::I64(n) => n.to_string(),
            Value::ISize(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Double(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.to_string(),
            Value::Bind(bind) => bind.to_string(),
            Value::Function(func) => func.to_string(),
            Value::Void => String::new(),
            // Value::Color(color) => color.to_string(),
            Value::Struct(s) => s.to_string(),
            Value::Vec(v) => format!(
                "{:?}",
                v.into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            ),
            Value::UnKnown(u) => u.to_string(),
            Value::Dep(dep) => dep.to_string(),
        };

        f.write_str(&res)
    }
}

#[cfg(test)]
mod test_value {}
