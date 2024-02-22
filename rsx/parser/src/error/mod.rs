use std::fmt::Display;

#[derive(Debug, PartialEq,Clone)]
pub enum Error{
    /// Tag
    TagStart,
    TagName,
    TagPropsKey,
    TagPropsValue,
    TagEnd,
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
    CommentType
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::TagStart => "tag start should be: `<`",
            Error::TagName => "tag name should use `-` or `_` for split",
            Error::TagPropsKey => "tag props' key should use `_` for split",
            Error::TagPropsValue => "tag props' value should in `Value`",
            Error::TagEnd => "tag end should be `>` for normal, `/>` for self close",
            Error::StyleType => "style type should use `.` | `#` | `&::`",
            Error::StyleName => "style name should use `_` for split",
            Error::StylePropsKey => "style props' key should use `_` for split",
            Error::StylePropsValue => "style props' value should in `Value`",
            Error::CommentType => "comment type should use `//` | `///` | `//!`",
        };
        f.write_fmt(format_args!("Parse Error:\n{}",msg))
    }
}


