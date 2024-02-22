mod comment;
mod nodes;
mod style;
mod tag;

use comment::Comments;
pub use nodes::ASTNodes;
use std::{collections::HashMap, fmt::Display};
pub use style::Style;
pub use tag::Tag;

use crate::{Value, SPACE};

pub type Props<'a> = Option<HashMap<&'a str, Value>>;

pub fn props_to_string(props: Props) -> String {
    match props {
        Some(props) =>  props
        .into_iter()
        .map(|(k, v)| format!(r#"{}="{}""#, k, v.to_string()))
        .collect::<Vec<String>>()
        .join(SPACE),
        None => String::new(),
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct ASTNode<'a> {
//     // node_type: TemplateNodeType,
//     // tag_name: Option<&'a str>,
//     node: Nodes,
//     properties: Option<HashMap<&'a str, Value>>,
//     children: Option<Vec<ASTNode<'a>>>,
//     parent: Option<Box<ASTNode<'a>>>,
// }

// impl<'a> ASTNode<'a> {
//     /// create a new node (tag | comment)
//     pub fn new(node: Nodes) -> Self {
//         Self {
//             // node_type,
//             // tag_name: Some(tag_name),
//             node,
//             properties: None,
//             children: None,
//             parent: None,
//         }
//     }
//     pub fn tag(tag: impl Into<Tag>) -> Self {
//         Self::new(Nodes::Tag(tag.into()))
//     }
//     pub fn comment(comment: impl Into<Comments>) -> Self {
//         Self::new(Nodes::Comment(comment.into()))
//     }
//     pub fn style(style: impl Into<Style>) -> Self {
//         Self::new(Nodes::Style(style.into()))
//     }
//     /// replace properties
//     pub fn properties(&mut self, properties: HashMap<&'a str, Value>) -> () {
//         self.properties.replace(properties);
//     }
//     /// replace children
//     pub fn children(&mut self, children: Option<Vec<ASTNode<'a>>>) -> () {
//         self.children = children;
//     }
//     pub fn get_node(&self) -> &Nodes {
//         &self.node
//     }
//     pub fn is_tag(&self) -> bool {
//         match self.get_node() {
//             Nodes::Tag(_) => true,
//             _ => false,
//         }
//     }
//     pub fn is_comment(&self) -> bool {
//         matches!(self.get_node(), Nodes::Comment(_))
//     }
//     pub fn is_style(&self) -> bool {
//         matches!(self.get_node(), Nodes::Style(_))
//     }
//     /// ## set parent
//     pub fn parent(&mut self, parent: ASTNode<'a>) -> () {
//         self.parent.replace(Box::new(parent));
//     }
// }

// // impl<'a> Display for ASTNode<'a> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

// //     }
// // }
