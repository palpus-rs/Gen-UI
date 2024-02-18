use std::fmt::Display;

use crate::{END_SIGN, SELF_END_SIGN, TAG_START};

#[derive(Debug, Clone, PartialEq)]
pub enum CloseType {
    /// <xxx /> -> `/>`
    SelfClosed,
    /// <xxx></xxx> -> `>`
    Normal,
}

impl Default for CloseType {
    fn default() -> Self {
        CloseType::Normal
    }
}

impl Display for CloseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            CloseType::SelfClosed => SELF_END_SIGN,
            CloseType::Normal => END_SIGN,
        };
        f.write_str(res)
    }
}

impl From<&str> for CloseType {
    fn from(value: &str) -> Self {
        match value {
            SELF_END_SIGN => CloseType::SelfClosed,
            END_SIGN => CloseType::Normal,
            _ => panic!("Invalid close type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    name: String,
    ty: CloseType,
}

impl Tag {
    pub fn new(name: &str, ty: CloseType) -> Self {
        Tag {
            name: name.to_string(),
            ty,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> &CloseType {
        &self.ty
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}",
            TAG_START,
            self.get_name(),
            self.get_type().to_string()
        ))
    }
}

impl From<(&str, CloseType)> for Tag {
    fn from(value: (&str, CloseType)) -> Self {
        Tag::new(value.0, value.1)
    }
}
