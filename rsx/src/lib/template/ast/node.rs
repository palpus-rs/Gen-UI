use super::{Comments, PropertyKey};
use std::{
    collections::HashMap,
    fmt::{write, Display},
};

use crate::lib::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateASTNode<'a> {
    node_type: TemplateNodeType,
    tag_name: Option<&'a str>,
    comment: Option<&'a str>,
    properties: Option<HashMap<PropertyKey<'a>, Value>>,
    children: Option<Vec<TemplateASTNode<'a>>>,
    parent: Option<Box<TemplateASTNode<'a>>>,
}

impl<'a> TemplateASTNode<'a> {
    /// create a new node (tag | comment)
    pub fn new(node_type: TemplateNodeType, tag_name: &'a str) -> Self {
        Self {
            node_type,
            tag_name:Some(tag_name),
            properties: None,
            children: None,
            parent: None,
            comment: None,
        }
    }
    pub fn tag(tag_name: &'a str)->Self{
        Self::new(TemplateNodeType::Tag, tag_name)
    }
    pub fn comment(comment:&'a str,comment_type:&'a str)->Self{
        Self {
            node_type:TemplateNodeType::Comment(comment_type.into()),
            tag_name:None,
            properties: None,
            children: None,
            parent: None,
            comment: Some(comment),
        }
    }
    /// replace properties
    pub fn properties(&mut self, properties: HashMap<PropertyKey<'a>, Value>) -> () {
        self.properties.replace(properties);
    }
    pub fn children(&mut self, children:Option<Vec<TemplateASTNode<'a>>>)->(){
        self.children = children;
    }
    pub fn get_tag_name(&self)->Option<&str>{
        self.tag_name
    }
    pub fn is_tag(&self)->bool{
        match self.node_type {
            TemplateNodeType::Tag => true,
            TemplateNodeType::Comment(_) =>false,
        }
    }
}

// impl<'a> Display for TemplateASTNode<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateNodeType {
    Tag,
    Comment(Comments),
}

impl Default for TemplateNodeType {
    fn default() -> Self {
        Self::Tag
    }
}

impl Display for TemplateNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            TemplateNodeType::Tag => String::from("tag"),
            TemplateNodeType::Comment(c) => c.to_string(),
        };
        f.write_str(&res)
    }
}
