use parser::{ASTNodes, ParseResult, Script};

use crate::{error::Errors, targets::makepad::{model::MakepadModel, PropRole}};

/// # Basic Visitor
/// - convert_template
/// - convert_script
/// - convert_style
pub trait Visitor {
    fn convert(ast:&ParseResult,source_name:&str)->Result<String,Errors>;
    fn convert_template( t: &ASTNodes,is_ref:bool)-> Result<MakepadModel,Errors>;
    fn convert_script(&self, sc: Script);
    // style just need to get the kv 
    // - Bind
    // - Function
    // - Normal
    fn convert_style(&self, s: &ASTNodes)-> Result<PropRole,Errors>;
}
