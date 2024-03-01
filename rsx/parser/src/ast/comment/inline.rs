use super::Comments;

#[derive(Debug,Clone,PartialEq)]
pub struct InlineComment<'a>(Comments<'a>);