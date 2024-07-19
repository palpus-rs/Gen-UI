use std::fmt::Display;

use gen_utils::error::Errors;
use quote::quote;
use syn::Block;

use crate::target::parse_script;

/// # Script
/// which is from `.gen` file, in `.gen` file, people can write rust code or ets code
/// - `<script lang="rust">` or `<script>` is rust code (default is rust code for makepad framework)
/// - `<script lang="ets">` is ets code (ets is now for ark HarmonyOs)
/// ---
/// if is rust code use Block to store, otherwise use String to store
#[derive(Debug, Clone, PartialEq)]
pub enum Script {
    /// rust code
    Rs(Block),
    /// ets code
    ETs(String),
    Other {
        lang: String,
        code: String,
    },
}

#[allow(dead_code)]
impl Script {
    /// is current script is empty or not
    pub fn is_empty(&self) -> bool {
        match self {
            Script::Rs(block) => block.stmts.is_empty(),
            Script::ETs(ets) => ets.is_empty(),
            Script::Other { code, .. } => code.is_empty(),
        }
    }
    // pub fn brace_token(&self) -> &Brace {
    //     &self.0.brace_token
    // }
    // pub fn ast(&self) -> &Vec<Stmt> {
    //     &self.0.stmts
    // }
    // pub fn to_origin(self) -> Block {
    //     match self {
    //         Script::Rs(rs) => todo!(),
    //         Script::ETs(ets) => todo!(),
    //     }
    // }
    // pub fn as_origin(&self) -> &Block {
    //     &self.0
    // }
    // pub fn as_origin_mut(&mut self) -> &mut Block {
    //     &mut self.0
    // }
}

impl From<Block> for Script {
    fn from(value: Block) -> Self {
        Script::Rs(value)
    }
}

impl TryFrom<(&str, Option<String>)> for Script {
    type Error = Errors;

    fn try_from(value: (&str, Option<String>)) -> Result<Self, Self::Error> {
        match value.1.as_ref() {
            Some(lang) => match lang.as_str() {
                "rust" | "rs" => {
                    let code =
                        parse_script(value.0).map_err(|e| Errors::ParseError(e.to_string()))?;
                    Ok(Script::Rs(code))
                }
                "ets" => Ok(Script::ETs(value.0.to_string())),
                other => Ok(Script::Other {
                    lang: other.to_string(),
                    code: value.0.to_string(),
                }),
            },
            None => Err(Errors::ParseError(
                "the tag must be script, current is not, please check!".to_string(),
            )),
        }
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Script::Rs(rs) => {
                // if is rust code use quote to format
                let res = quote! {
                    #rs
                };
                // remove `{}`
                let convert_str = res.to_string();
                let convert_str = &convert_str[1..convert_str.len() - 1];
                f.write_str(convert_str.trim())
            }
            Script::ETs(ets) => f.write_str(ets), // otherwise use origin format
            Script::Other { code, .. } => f.write_str(code),
        }
    }
}
