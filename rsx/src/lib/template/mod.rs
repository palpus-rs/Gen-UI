mod ast;
mod parser;

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateTag<'a> {
    TagName(&'a str),
    Property(Property<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property<'a> {
    key: &'a str,
    value: Value,
}
