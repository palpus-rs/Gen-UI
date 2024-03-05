use parser::{ASTNodes, Script};

/// Basic Visitor
pub trait Visitor {
    fn convert_template(&self, t: &ASTNodes);
    fn convert_script(&self, t: Script);
    fn convert_style(&self, t: &ASTNodes);
}
