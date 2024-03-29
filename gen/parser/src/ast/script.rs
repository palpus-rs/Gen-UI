use std::fmt::Display;

use quote::quote;
use syn::{token::Brace, Block, Stmt};

use crate::target::parse_script;

#[derive(Debug, Clone, PartialEq)]
pub struct Script(Block);

#[allow(dead_code)]
impl Script {
    pub fn brace_token(&self) -> &Brace {
        &self.0.brace_token
    }
    pub fn ast(&self) -> &Vec<Stmt> {
        &self.0.stmts
    }
    pub fn to_origin(self) -> Block {
        self.0
    }
    pub fn as_origin(&self) -> &Block {
        &self.0
    }
}

impl From<Block> for Script {
    fn from(value: Block) -> Self {
        Script(value)
    }
}

impl<'a> TryFrom<&'a str> for Script {
    type Error = crate::error::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match parse_script(value) {
            Ok(block) => Ok(Script(block)),
            Err(e) => Err(e),
        }
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block = self.as_origin();
        let res = quote! {
            #block
        };
        // remove `{}`
        let convert_str = res.to_string();
        let convert_str = &convert_str[1..convert_str.len() - 1];
        f.write_str(convert_str.trim())
    }
}

#[cfg(test)]
mod test_script {
    use super::Script;

    #[test]
    fn test_display() {
        let code = r#"
        let mut counter:usize = 0_usize;

        let mut click = ||{
            counter += 1;
        };
        "#;

        match Script::try_from(code) {
            Ok(ast) => {
                assert_eq!(
                    ast.to_string().as_str(),
                    "let mut counter : usize = 0_usize ; let mut click = | | { counter += 1 ; } ;"
                );
            }
            Err(e) => {
                dbg!(e.to_string());
            }
        };
    }
}
