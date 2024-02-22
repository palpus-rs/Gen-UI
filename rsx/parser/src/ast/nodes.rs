use std::fmt::Display;
use crate::{error::Error, template::parse_template};

use super::{Tag,Comments,Style};


#[derive(Debug, Clone, PartialEq)]
pub enum ASTNodes<'a> {
    /// ### template tag 
    /// - <template>
    /// - <script>
    /// - <style>
    /// - <any_component>
    /// - ...
    Tag(Box<Tag<'a>>),
    /// ### Comment
    /// display everywhere
    /// - ///
    /// - //
    /// - //!
    Comment(Comments<'a>),
    /// ### Style (Properties)
    /// - .
    /// - #
    /// - &::
    Style(Style),
}

impl<'a> ASTNodes<'a> {
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }
    pub fn is_style(&self) -> bool {
        matches!(self,Self::Style(_))
    }
    pub fn parse_template(input:&str) -> Vec<ASTNodes>{
        parse_template(input)
    }
    pub fn parse(input:&str)->Result<Vec<ASTNodes>, Error>{
        
    }
}


impl<'a> Display for ASTNodes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ASTNodes::Tag(t) => t.to_string(),
            ASTNodes::Comment(c) => c.to_string(),
            ASTNodes::Style(s) => s.to_string(),
            
        };
        f.write_str(&res)
    }
}
