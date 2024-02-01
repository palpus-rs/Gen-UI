mod ast;
mod parser;

pub use ast::{Comments, TemplateASTNode};
pub use parser::parse_template;
