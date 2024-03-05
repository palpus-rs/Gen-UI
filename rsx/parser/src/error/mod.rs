use core::str;
use std::{error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Errors {
    ParseError(String),
    TemplateParseRemain(String),
    ParseTargetEmpty,
    ParseTargetError(String),
    /// Tag
    TagStart,
    TagName,
    TagPropsKey,
    TagPropsValue,
    TagEnd,
    EndTag,
    /// Style
    /// type :
    /// - .
    /// - #
    /// - &::
    StyleType,
    StyleName,
    StylePropsKey,
    StylePropsValue,
    /// Comment
    CommentType,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Errors::ParseError(e) => e.to_string(),
            Errors::TagStart => "tag start should be: `<`".to_string(),
            Errors::TagName => "tag name should use `-` or `_` for split".to_string(),
            Errors::TagPropsKey => "tag props' key should use `_` for split".to_string(),
            Errors::TagPropsValue => "tag props' value should in `Value`".to_string(),
            Errors::TagEnd => "tag end should be `>` for normal, `/>` for self close".to_string(),
            Errors::StyleType => "style type should use `.` | `#` | `&::`".to_string(),
            Errors::StyleName => "style name should use `_` for split".to_string(),
            Errors::StylePropsKey => "style props' key should use `_` for split".to_string(),
            Errors::StylePropsValue => "style props' value should in `Value`".to_string(),
            Errors::CommentType => "comment type should use `//` | `///` | `//!`".to_string(),
            Errors::EndTag => "can not find end tag, please check".to_string(),
            Errors::TemplateParseRemain(remain) => format!(
                "template parse still has remain: {}. Not in compliance with standard writing",
                remain
            ),
            Errors::ParseTargetEmpty => "`ParseTarget` is empty which means the current rsx file is empty, do not need to convert to AST".to_string(),
            Errors::ParseTargetError(e) => e.to_string(),
        };
        f.write_str(&msg)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error(Errors);

#[allow(dead_code)]
impl Error {
    pub fn convert(e: Errors) -> Self {
        Self(e)
    }
    pub fn new(msg: &str) -> Self {
        Error(Errors::ParseError(msg.to_string()))
    }
    pub fn parse_error(msg: &str) -> Self {
        Error(Errors::ParseError(msg.to_string()))
    }
    pub fn template_parser_remain(remain: &str) -> Self {
        Error(Errors::TemplateParseRemain(remain.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Parse Error:\n{}", self.0.to_string()))
    }
}

impl error::Error for Error {}
