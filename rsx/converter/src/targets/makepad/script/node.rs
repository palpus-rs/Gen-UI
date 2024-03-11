use std::fmt::Display;

use super::NodeVariable;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptNode<'a>{
    Variable(NodeVariable<'a>),
    Function(String),

}

impl<'a> Display for ScriptNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptNode::Variable(v) => f.write_str(v.to_string().as_str()),
            ScriptNode::Function(_) => todo!(),
        }
    }
}