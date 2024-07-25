use std::{collections::BTreeMap, fmt::Display};

use gen_utils::error::Errors;

use crate::PropertyKeyType;

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    name: String,
    fields: BTreeMap<String, Value>,
    is_anonymous: bool,
}

impl Struct {
    pub fn new(name: &str) -> Self {
        Struct {
            name: name.to_string(),
            fields: BTreeMap::new(),
            is_anonymous: false,
        }
    }
    pub fn insert(&mut self, key: &str, value: Value) {
        self.fields.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.is_anonymous = false;
    }
}

impl TryFrom<serde_json::Value> for Struct {
    type Error = Errors;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let mut fields = BTreeMap::new();
        if let serde_json::Value::Object(obj) = value {
            for (k, v) in obj {
                fields.insert(k, Value::try_from(v)?);
            }
        }
        Ok(Struct {
            name: String::new(),
            fields,
            is_anonymous: true,
        })
    }
}

impl From<Vec<(&str, &str)>> for Struct {
    fn from(value: Vec<(&str, &str)>) -> Self {
        let mut fields = BTreeMap::new();
        for (k, v) in value {
            fields.insert(k.to_string(), Value::from(v));
        }
        Struct {
            name: String::new(),
            fields,
            is_anonymous: true,
        }
    }
}

impl From<Vec<(&str, (Value, PropertyKeyType))>> for Struct{
    fn from(value: Vec<(&str, (Value, PropertyKeyType))>) -> Self {
        let mut fields = BTreeMap::new();
        for (k, (v, _)) in value {
            fields.insert(k.to_string(), v);
        }
        Struct {
            name: String::new(),
            fields,
            is_anonymous: true,
        }
    }
}

impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields = String::new();
        for (k, v) in &self.fields {
            fields.push_str(&format!("{}: {}, ", k, v));
        }
        if self.is_anonymous {
            write!(f, "{{{}}}", fields)
        } else {
            write!(f, "{} {{{}}}", self.name, fields)
        }
    }
}
