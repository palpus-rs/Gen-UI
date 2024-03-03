use crate::{CONST, LET};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Const,
    Let,
}

impl From<&str> for VariableType {
    fn from(value: &str) -> Self {
        match value {
            LET => VariableType::Let,
            CONST => VariableType::Const,
            _ => panic!("Invalid variable type"),
        }
    }
}
