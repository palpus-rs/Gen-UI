mod node;
mod variable;
pub use node::*;
pub use variable::*;


use std::fmt::Display;



#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ConvertScript<'a> {
    Rust(String),
    /// need to join('\n')
    MakepadRS(Vec<ScriptNode<'a>>),
}

impl<'a> Display for ConvertScript<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertScript::Rust(rs) => f.write_str(rs),
            ConvertScript::MakepadRS(stmts) => {
                let block = stmts
                    .into_iter()
                    .map(|stmt| stmt.to_string())
                    .collect::<String>();
                f.write_fmt(format_args!("{}", block))
            }
        }
    }
}

