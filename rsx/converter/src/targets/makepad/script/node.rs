use std::fmt::Display;

use syn::{Item, ItemStruct, Stmt};

use crate::targets::makepad::action::MakepadAction;

use super::NodeVariable;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptNode {
    Variable(NodeVariable),
    Function(MakepadAction),
    Struct(ItemStruct)
}

#[allow(dead_code)]
impl ScriptNode {
    pub fn get_var(&self) -> Option<&NodeVariable> {
        match self {
            ScriptNode::Variable(v) => Some(v),
            _ => None,
        }
    }
}

impl Display for ScriptNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptNode::Variable(v) => f.write_str(v.to_string().as_str()),
            ScriptNode::Function(_) => todo!(),
            ScriptNode::Struct(_) => todo!(),
            
        }
    }
}
