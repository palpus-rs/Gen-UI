use std::{collections::HashMap, fmt::Display};

use crate::{Value, END_SIGN, SELF_END_SIGN, TAG_START};

use super::{props_to_string, Props};

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
    props: Props
}

impl Tag {
    pub fn new(name: &str, ty: CloseType,props:Props) -> Self {
        Tag {
            name: name.to_string(),
            ty,
            props,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> CloseType {
        self.ty.clone()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{} {} {}",
            TAG_START,
            self.get_name(),
            props_to_string(self.props),
            self.get_type().to_string()
        ))
    }
}

impl From<(&str, CloseType)> for Tag {
    fn from(value: (&str, CloseType)) -> Self {
        Tag::new(value.0, value.1)
    }
}

#[cfg(test)]
mod test_tag {
    use super::{CloseType, Tag};

    #[test]
    fn get_name() {
        let tag_normal = Tag::new("input", CloseType::Normal);
        let tag_close_self = Tag::new("select", CloseType::SelfClosed);

        assert_eq!(tag_normal.get_name(), "input");
        assert_eq!(tag_close_self.get_name(), "select");
    }
    #[test]
    fn get_type() {
        let tag_normal = Tag::new("input", CloseType::Normal);
        let tag_close_self = Tag::new("select", CloseType::SelfClosed);

        assert_eq!(tag_normal.get_type(), CloseType::Normal);
        assert_eq!(tag_close_self.get_type(), CloseType::SelfClosed);
    }
    #[test]
    fn display() {
        let tag_normal = Tag::new("input", CloseType::Normal);
        let tag_close_self = Tag::new("select", CloseType::SelfClosed);

        assert_eq!(tag_normal.to_string().as_str(), "<input>");
        assert_eq!(tag_close_self.to_string().as_str(), "<select/>");
    }
}
