use std::fmt::Display;

use crate::lib::Value;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct PropertyKey<'a> {
    key: &'a str,
    key_type: PropertyKeyType,
}

impl<'a> PropertyKey<'a> {
    pub fn new(key_type: PropertyKeyType, key: &'a str) -> Self {
        Self { key, key_type }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PropertyKeyType {
    Normal,
    /// :xxx
    Bind,
    /// @xxx
    Function,
}

impl PropertyKeyType {
    pub fn to_value(&self,value:&str)->Value{
        let value = value.to_string();
        match self {
            PropertyKeyType::Normal => Value::String(value),
            PropertyKeyType::Bind => Value::Bind(value),
            PropertyKeyType::Function => Value::Function(value),
        }
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
