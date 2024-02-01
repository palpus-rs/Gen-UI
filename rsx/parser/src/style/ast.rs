use std::{collections::HashMap, fmt::Display};

use crate::{Value, STYLE_CLASS, STYLE_ID, STYLE_PESUDO};

#[derive(Debug, Clone, PartialEq)]
pub struct StyleASTNode<'a> {
    style_type: StyleNodeType,
    name: Option<&'a str>,
    comment: Option<&'a str>,
    properties: Option<HashMap<&'a str, Value>>,
    children: Option<Vec<StyleASTNode<'a>>>,
    parent: Option<Box<StyleASTNode<'a>>>,
}

impl<'a> StyleASTNode<'a> {
    pub fn new(style_type: StyleNodeType, name: &'a str) -> Self {
        Self {
            style_type,
            name: Some(name),
            comment: None,
            properties: None,
            children: None,
            parent: None,
        }
    }
    pub fn class(name: &'a str) -> Self {
        Self::new(StyleNodeType::Class, name)
    }
    pub fn id(name: &'a str) -> Self {
        Self::new(StyleNodeType::Id, name)
    }
    pub fn pseudo(name: &'a str) -> Self {
        Self::new(StyleNodeType::Pseudo, name)
    }
    /// replace properties
    pub fn properties(&mut self, properties: HashMap<&'a str, Value>) -> () {
        self.properties.replace(properties);
    }
    /// replace children
    pub fn children(&mut self, children: Option<Vec<StyleASTNode<'a>>>) -> () {
        self.children = children;
    }
    pub fn parent(&mut self, parent: StyleASTNode<'a>) -> () {
        self.parent.replace(Box::new(parent));
    }
    pub fn get_name(&self) -> Option<&str> {
        self.name
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum StyleNodeType {
    // class: `.`
    Class,
    // id: `#`
    Id,
    // Pseudo: `::`
    Pseudo,
}

impl Default for StyleNodeType {
    fn default() -> Self {
        Self::Class
    }
}

impl StyleNodeType {
    pub fn is_class(&self) -> bool {
        matches!(self, Self::Class)
    }
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id)
    }
    pub fn is_pseudo(&self) -> bool {
        matches!(self, Self::Pseudo)
    }
}

impl From<&str> for StyleNodeType {
    fn from(value: &str) -> Self {
        match value {
            STYLE_CLASS => StyleNodeType::Class,
            STYLE_ID => StyleNodeType::Id,
            STYLE_PESUDO => StyleNodeType::Pseudo,
            _ => panic!("Invalid style"),
        }
    }
}

impl Display for StyleNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            StyleNodeType::Class => STYLE_CLASS,
            StyleNodeType::Id => STYLE_ID,
            StyleNodeType::Pseudo => STYLE_PESUDO,
        };
        f.write_str(res)
    }
}
