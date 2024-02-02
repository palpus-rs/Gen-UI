use std::fmt::Display;
use super::{Tag,Comments,Style};


#[derive(Debug, Clone, PartialEq)]
pub enum Nodes {
    /// ### template tag 
    /// - <template>
    /// - <script>
    /// - <style>
    /// - ...
    Tag(Tag),
    /// ### Comment
    /// display everywhere
    /// - ///
    /// - //
    /// - //!
    Comment(Comments),
    /// ### Style (Properties)
    /// - .
    /// - #
    /// - &::
    Style(Style),
}

impl Nodes {
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }
    pub fn is_style(&self) -> bool {
        matches!(self,Self::Style(_))
    }
}


impl Display for Nodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Nodes::Tag(t) => String::from("tag"),
            Nodes::Comment(c) => c.to_string(),
            Nodes::Style(s) => todo!(),
            
        };
        f.write_str(&res)
    }
}
