use std::fmt::Display;

use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PropertyKeyType {
    Normal,
    /// :xxx
    Bind,
    /// @xxx
    Function,
}

#[allow(dead_code)]
impl PropertyKeyType {
    pub fn to_value(&self, value: &str) -> Value {
        let value = value.to_string();
        match self {
            PropertyKeyType::Normal => Value::String(value),
            PropertyKeyType::Bind => Value::Bind(value),
            PropertyKeyType::Function => Value::Function(value.into()),
        }
    }
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
    pub fn is_bind(&self) -> bool {
        matches!(self, Self::Bind)
    }
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function)
    }
}

impl Default for PropertyKeyType {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for PropertyKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            PropertyKeyType::Normal => "normal",
            PropertyKeyType::Bind => "bind",
            PropertyKeyType::Function => "function",
        };
        f.write_str(res)
    }
}

impl From<&str> for PropertyKeyType {
    fn from(value: &str) -> Self {
        match value {
            "" => PropertyKeyType::Normal,
            ":" => PropertyKeyType::Bind,
            "@" => PropertyKeyType::Function,
            _ => panic!("Invalid property key"),
        }
    }
}
