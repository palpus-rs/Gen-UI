mod handler;
mod node;
mod ty;
mod utils;
mod variable;

pub use node::*;
pub use ty::*;
pub use utils::*;
pub use variable::*;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ConvertScript {
    Rust(String),
    /// need to join('\n')
    MakepadRS(Vec<ScriptNode>),
}

impl ConvertScript {
    pub fn get_makepad_vars(&self) -> Option<Vec<&NodeVariable>> {
        if let ConvertScript::MakepadRS(mrss) = self {
            let mut vars = vec![];
            for mr in mrss {
                if let ScriptNode::Variable(var) = mr {
                    vars.push(var);
                }
            }
            return if vars.is_empty() { None } else { Some(vars) };
        }
        panic!("not a MakepadRS")
    }
    pub fn get_makepad_rs(&self) -> Vec<&ScriptNode> {
        if let ConvertScript::MakepadRS(mrss) = self {
            return mrss.iter().collect();
        }
        panic!("not a MakepadRS")
    }
}

impl Display for ConvertScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertScript::Rust(rs) => f.write_str(rs),
            ConvertScript::MakepadRS(stmts) => {
                let block = stmts
                    .into_iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                f.write_fmt(format_args!("{}", block))
            }
        }
    }
}
