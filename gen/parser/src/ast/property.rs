use std::{collections::HashMap, fmt::Display};

use gen_utils::common::tokenizer::SPACE;

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
            PropertyKeyType::Normal => Value::UnKnown(value),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropsKey {
    name: String,
    /// same as function
    /// judge the use place (template|style)
    /// has behave differently
    is_style: bool,
    ty: PropertyKeyType,
}

impl PropsKey {
    pub fn new(name: &str, is_style: bool, ty: PropertyKeyType) -> Self {
        PropsKey {
            name: name.to_string(),
            is_style,
            ty,
        }
    }
    /// ## new props key
    /// new a props key in template or script tag which is type normal
    pub fn new_tag_normal(name: &str) -> Self {
        Self {
            name: name.to_string(),
            is_style: false,
            ty: PropertyKeyType::Normal,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &PropertyKeyType {
        &self.ty
    }
    pub fn is_bind(&self) -> bool {
        self.ty.is_bind()
    }
    pub fn is_normal(&self) -> bool {
        self.ty.is_normal()
    }
    pub fn is_fn(&self) -> bool {
        self.ty.is_function()
    }
}

impl Display for PropsKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ty {
            PropertyKeyType::Normal => f.write_str(self.name()),
            PropertyKeyType::Bind => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!(":{}", self.name()))
                }
            }
            PropertyKeyType::Function => {
                if self.is_style {
                    f.write_str(self.name())
                } else {
                    f.write_fmt(format_args!("@{}", self.name()))
                }
            }
        }
    }
}

pub type Props = Option<HashMap<PropsKey, Value>>;

pub fn props_to_string<'a, F>(props: Props, format: F) -> String
where
    F: FnMut((PropsKey, Value)) -> String,
{
    match props {
        Some(props) => props
            .into_iter()
            .map(format)
            .collect::<Vec<String>>()
            .join(SPACE),
        None => String::new(),
    }
}

pub fn props_to_template_string(props: Props) -> String {
    props_to_string(props, |(k, v)| {
        format!(r#"{}="{}""#, k.to_string(), v.to_string())
    })
}

pub fn props_to_style_string(props: Props) -> String {
    props_to_string(props, |(k, v)| {
        format!(r#"{}: {};"#, k.to_string(), v.to_string())
    })
}
