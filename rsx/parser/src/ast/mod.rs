mod comment;
mod nodes;
mod style;
mod tag;

use comment::offline::OfflineComment;
pub use nodes::ASTNodes;
use nom::{bytes::complete::{tag, take_until}, sequence::delimited, IResult};
use std::{collections::HashMap, fmt::Display};
pub use style::Style;
pub use tag::Tag;

use crate::{Value, SPACE};

pub type Props<'a> = Option<HashMap<&'a str, Value>>;

pub fn props_to_string(props: Props) -> String {
    match props {
        Some(props) => props
            .into_iter()
            .map(|(k, v)| format!(r#"{}="{}""#, k, v.to_string()))
            .collect::<Vec<String>>()
            .join(SPACE),
        None => String::new(),
    }
}

/// Parse Strategy
pub enum Strategy {
    /// an empty file
    None,
    /// only has template tag
    SingleTemplate,
    /// only has rust script
    SingleScript,
    /// only has style tag
    SingleStyle,
    /// no template, rust script, style
    /// only comment (should with signatures)
    SingleComment,
    /// template with rust script
    TemplateScript,
    /// template with style
    TemplateStyle,
    /// template with comment
    TemplateComment,
    /// script with comment
    ScriptComment,
    /// style with comment
    StyleComment,
    TemplateScriptComment,
    TemplateStyleComment,
    /// has all
    All,
    Error(String),
}

/// # Parse Target
/// The target which will be parsed
///
/// Through this structure, you can obtain the page structure
///  
/// ## how to get
/// use nom to split the rsx file
/// ## target check
/// When calling to determine the existence of fields in the parsing target, the actual content will be determined to be empty or not
/// > reject cheat syntax
pub struct ParseTarget {
    /// content of template tag
    template: Option<String>,
    /// content of script tag
    script: Option<String>,
    /// content of style tag
    style: Option<String>,
    /// after parse the core 3 tag parser will consider the other remains are comment
    /// try to use comment parser to parse the remains
    /// if not have any allowed comment signatures --> panic!
    comment: Option<Vec<OfflineComment>>,
}

impl ParseTarget {
    pub fn template(&self) -> Option<&String> {
        self.template.as_ref()
    }
    pub fn script(&self) -> Option<&String> {
        self.script.as_ref()
    }
    pub fn style(&self) -> Option<&String> {
        self.style.as_ref()
    }
    pub fn comment(&self) -> Option<&Vec<OfflineComment>> {
        self.comment.as_ref()
    }
    pub fn has_template(&self) -> bool {
        has_target(self.template())
    }
    pub fn has_script(&self) -> bool {
        has_target(self.script())
    }
    pub fn has_style(&self) -> bool {
        has_target(self.style())
    }
    /// judge whether has other comments
    pub fn has_comment(&self) -> bool {
        match self.comment() {
            Some(v) => !v.is_empty(),
            None => false,
        }
    }
    pub fn has(&self) -> (bool, bool, bool, bool) {
        (
            self.has_template(),
            self.has_script(),
            self.has_style(),
            self.has_comment(),
        )
    }
    pub fn parse_template(input:&str)->IResult<&str,&str>{
        delimited(tag("<template>"),take_until("</template>"),tag("</template>"))(input)
    }
    pub fn target_strategy(&self) -> Strategy {
        match self.has() {
            (true, true, true, true) | (true, true, true, false) => Strategy::All,
            (true, true, false, true) => Strategy::TemplateScriptComment,
            (true, true, false, false) => Strategy::TemplateScript,
            (true, false, true, true) => Strategy::TemplateStyleComment,
            (true, false, true, false) => Strategy::TemplateStyle,
            (true, false, false, true) => Strategy::TemplateComment,
            (true, false, false, false) => Strategy::SingleTemplate,
            (false, true, true, true) | (false, true, true, false) => {
                Strategy::Error(String::from(
                    "RSX Parse Strategy Error: There is no such strategy `Script` + `Style`",
                ))
            }
            (false, true, false, true) => Strategy::ScriptComment,
            (false, true, false, false) => Strategy::SingleScript,
            (false, false, true, true) => Strategy::StyleComment,
            (false, false, true, false) => Strategy::SingleStyle,
            (false, false, false, true) => Strategy::SingleComment,
            (false, false, false, false) => Strategy::None,
        }
    }
}

fn has_target(target: Option<&String>) -> bool {
    match target {
        Some(v) => !v.is_empty(),
        None => false,
    }
}

// /// parse whole rsx file
// /// 1. use nom to get the part of the rsx file (parse to ParseTarget)
// ///     1. no <template> tag and no <style> tag  -->  parse as rust script (1 thread)
// ///     2. no <template> tag and no rust script has <style> tag  -->  parse as style (1 thread)
// ///     3. no <style> tag and no rust script has <template> tag  -->  parse as template (1 thread)
// ///     4. has <template> tag and rust script no <style> tag --> parse as template_script (2 thread)
// ///     5. has 3 tag --> parse as whole rsx (3 thread)
// impl TryFrom<&str> for ParseTarget {
//     type Error = crate::error::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {

//     }
// }

#[cfg(test)]
mod ast_test {
    #[test]
    fn parse_target() {}
}

// impl Display for ParseTarget {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         /// <template>\n{}\n</template>\n\n<script>\n{}\n</script>\n\n<style>{}</style>
//         f.write_fmt(format_args!("",))
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct ASTNode<'a> {
//     // node_type: TemplateNodeType,
//     // tag_name: Option<&'a str>,
//     node: Nodes,
//     properties: Option<HashMap<&'a str, Value>>,
//     children: Option<Vec<ASTNode<'a>>>,
//     parent: Option<Box<ASTNode<'a>>>,
// }

// impl<'a> ASTNode<'a> {
//     /// create a new node (tag | comment)
//     pub fn new(node: Nodes) -> Self {
//         Self {
//             // node_type,
//             // tag_name: Some(tag_name),
//             node,
//             properties: None,
//             children: None,
//             parent: None,
//         }
//     }
//     pub fn tag(tag: impl Into<Tag>) -> Self {
//         Self::new(Nodes::Tag(tag.into()))
//     }
//     pub fn comment(comment: impl Into<Comments>) -> Self {
//         Self::new(Nodes::Comment(comment.into()))
//     }
//     pub fn style(style: impl Into<Style>) -> Self {
//         Self::new(Nodes::Style(style.into()))
//     }
//     /// replace properties
//     pub fn properties(&mut self, properties: HashMap<&'a str, Value>) -> () {
//         self.properties.replace(properties);
//     }
//     /// replace children
//     pub fn children(&mut self, children: Option<Vec<ASTNode<'a>>>) -> () {
//         self.children = children;
//     }
//     pub fn get_node(&self) -> &Nodes {
//         &self.node
//     }
//     pub fn is_tag(&self) -> bool {
//         match self.get_node() {
//             Nodes::Tag(_) => true,
//             _ => false,
//         }
//     }
//     pub fn is_comment(&self) -> bool {
//         matches!(self.get_node(), Nodes::Comment(_))
//     }
//     pub fn is_style(&self) -> bool {
//         matches!(self.get_node(), Nodes::Style(_))
//     }
//     /// ## set parent
//     pub fn parent(&mut self, parent: ASTNode<'a>) -> () {
//         self.parent.replace(Box::new(parent));
//     }
// }

// // impl<'a> Display for ASTNode<'a> {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

// //     }
// // }
