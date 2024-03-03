pub mod comment;
mod nodes;
mod property;
mod style;
mod tag;

use comment::offline::OfflineComment;
pub use nodes::ASTNodes;

pub use property::*;
use std::{collections::HashMap, fmt::Display};
pub use style::Style;
pub use tag::Tag;

use crate::{
    ast::comment::position::OfflinePosition,
    common::{parse_all, trim},
    Value, SPACE,
};

pub type Props<'a> = Option<HashMap<&'a str, Value>>;

pub fn props_to_string<F>(props: Props, format: F) -> String
where
    F: FnMut((&str, Value)) -> String,
{
    match props {
        Some(props) => props
            .into_iter()
            .map(format)
            .collect::<Vec<String>>()
            .join(SPACE),
        None => String::new(),
    }
}

pub fn props_to_template_string(props: Props) -> String {
    props_to_string(props, |(k, v)| format!(r#"{}="{}""#, k, v.to_string()))
}

pub fn props_to_style_string(props: Props) -> String {
    props_to_string(props, |(k, v)| format!(r#"{}: {};"#, k, v.to_string()))
}

/// Parse Strategy
/// Convert ParseTarget To AST
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

#[derive(Debug, Clone, PartialEq)]
pub enum Targets<'a> {
    Template(&'a str),
    Script(&'a str),
    Style(&'a str),
    Comment(OfflineComment<'a>),
}

impl<'a> Targets<'a> {
    pub fn is_template(&self) -> bool {
        matches!(self, Targets::Template(_))
    }
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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParseTarget<'a> {
    /// content of template tag
    template: Option<&'a str>,
    /// content of script tag
    script: Option<&'a str>,
    /// content of style tag
    style: Option<&'a str>,
    /// after parse the core 3 tag parser will consider the other remains are comment
    /// try to use comment parser to parse the remains
    /// if not have any allowed comment signatures --> panic!
    comment: Option<Vec<OfflineComment<'a>>>,
}

#[allow(dead_code)]
impl<'a> ParseTarget<'a> {
    pub fn set_template(&mut self, template: &'a str) {
        let _ = self.template.replace(template);
    }
    pub fn set_script(&mut self, script: &'a str) {
        let _ = self.script.replace(script);
    }
    pub fn set_style(&mut self, style: &'a str) {
        let _ = self.style.replace(style);
    }
    pub fn set_comment(&mut self, comment: Vec<OfflineComment<'a>>) {
        let _ = self.comment.replace(comment);
    }
    pub fn push_comment(&mut self, comment: OfflineComment<'a>) {
        match &mut self.comment {
            Some(c) => c.push(comment),
            None => {
                let _ = self.comment.replace(vec![comment]);
            }
        }
    }
    pub fn template(&self) -> Option<&'a str> {
        self.template
    }
    pub fn script(&self) -> Option<&'a str> {
        self.script
    }
    pub fn style(&self) -> Option<&'a str> {
        self.style
    }
    pub fn comment(&self) -> Option<&Vec<OfflineComment>> {
        self.comment.as_ref()
    }
    pub fn has_template(&self) -> (bool, bool) {
        has_target(self.template())
    }
    pub fn has_script(&self) -> (bool, bool) {
        has_target(self.script())
    }
    pub fn has_style(&self) -> (bool, bool) {
        has_target(self.style())
    }
    /// judge whether has other comments
    pub fn has_comment(&self) -> (bool, bool) {
        match self.comment() {
            Some(v) => (!v.is_empty(), false),
            None => (false, true),
        }
    }
    pub fn has(&self) -> (bool, bool, bool, bool) {
        (
            self.has_template().0,
            self.has_script().0,
            self.has_style().0,
            self.has_comment().0,
        )
    }
    /// # handle Self to be better
    /// Call in TryFrom trait
    /// ## which need to handle
    /// is empty but not none
    pub fn handle_self(&mut self) {
        match self.has_template() {
            (false, false) => {
                self.template = None;
            }
            _ => {}
        }
        match self.has_script() {
            (false, false) => {
                self.script = None;
            }
            _ => {}
        }
        match self.has_style() {
            (false, false) => {
                self.style = None;
            }
            _ => {}
        }
    }
    /// Get ParseTarget Convert to AST Strategy
    /// This strategy affects how many threads are used for conversion
    ///
    /// 1. no <template> tag and no <style> tag  -->  parse as rust script (1 thread)
    /// 2. no <template> tag and no rust script has <style> tag  -->  parse as style (1 thread)
    /// 3. no <style> tag and no rust script has <template> tag  -->  parse as template (1 thread)
    /// 4. has <template> tag and rust script no <style> tag --> parse as template_script (2 thread)
    /// 5. has 3 tag --> parse as whole rsx (3 thread)
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

/// parse whole rsx file from `Vec<Targets>` to `ParseTarget`
impl<'a> TryFrom<Vec<Targets<'a>>> for ParseTarget<'a> {
    type Error = crate::error::Error<'a>;

    fn try_from(value: Vec<Targets<'a>>) -> Result<Self, Self::Error> {
        return if value.is_empty() {
            Err(crate::error::Error::new("The current file has no content. It should be removed to ensure your program has clean file tree!"))
        } else {
            let mut parse_target = ParseTarget::default();
            let mut template_count = 0_u32;
            let mut script_count = 0_u32;
            let mut style_count = 0_u32;
            for target in value {
                if is_multi_nodes(template_count, script_count, style_count) {
                    match target {
                        Targets::Template(t) => {
                            template_count += 1;
                            parse_target.set_template(t);
                        }
                        Targets::Script(sc) => {
                            script_count += 1;
                            parse_target.set_script(sc);
                        }
                        Targets::Style(s) => {
                            style_count += 1;
                            parse_target.set_style(s);
                        }
                        Targets::Comment(c) => parse_target.push_comment(c),
                    }
                } else {
                    return Err(crate::error::Error::new("Abnormal number of nodes, there is more than one `template` | `script` | `style` node in the file!"));
                }
            }
            let _ = parse_target.handle_self();
            Ok(parse_target)
        };
    }
}

/// parse whole rsx file from `&str` to `ParseTarget`
/// recommended to use this method to parse rsx file directly
impl<'a> TryFrom<&'a str> for ParseTarget<'a> {
    type Error = crate::error::Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        return if value.trim().is_empty() {
            Err(crate::error::Error::new("The current file has no content. It should be removed to ensure your program has clean file tree!"))
        } else {
            let (remain, res) = trim(parse_all)(value).unwrap();
            if remain.is_empty() {
                // parse res to ParseTarget
                return ParseTarget::try_from(res);
            } else {
                return Err(crate::error::Error::new("Parsing file exception. The current file contains content that is not covered by processed tags. If it is a rust script, please wrap it in a `<script>` tag"));
            }
        };
    }
}

impl<'a> Display for ParseTarget<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let has_comment = self.has_comment().0;
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "{}\n",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveTemplate)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_template().0 {
            let _ = f.write_fmt(format_args!(
                "<template>\n{}\n</template>\n",
                self.template().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "\n{}",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveScript)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_script().0 {
            let _ = f.write_fmt(format_args!(
                "\n<script>\n{}\n</script>\n",
                self.script().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_fmt(format_args!(
                "\n{}",
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::AboveStyle)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        if self.has_style().0 {
            let _ = f.write_fmt(format_args!(
                "\n<style>\n{}\n</style>\n",
                self.style().unwrap()
            ));
        }
        if has_comment {
            let _ = f.write_str(
                &self
                    .comment()
                    .unwrap()
                    .iter()
                    .filter(|item| item.position() == OfflinePosition::End)
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        f.write_str("\n")
    }
}

/// check whether the target is empty
/// ## return
/// `(bool, bool)` means:
/// 1. bool: is empty?
/// 2. bool: is Option::None?
fn has_target(target: Option<&str>) -> (bool, bool) {
    match target {
        Some(v) => (!v.is_empty(), false),
        None => (false, true),
    }
}

fn is_multi_nodes(t: u32, sc: u32, s: u32) -> bool {
    (t <= 1) && (sc <= 1) && (s <= 1)
}

// fn offline_comments_to_string(comments:Vec<&OfflineComment>)->String{
//     comments.i
// }

/// parse whole rsx file
/// 1. use nom to get the part of the rsx file (parse to ParseTarget)
///     1. no <template> tag and no <style> tag  -->  parse as rust script (1 thread)
///     2. no <template> tag and no rust script has <style> tag  -->  parse as style (1 thread)
///     3. no <style> tag and no rust script has <template> tag  -->  parse as template (1 thread)
///     4. has <template> tag and rust script no <style> tag --> parse as template_script (2 thread)
///     5. has 3 tag --> parse as whole rsx (3 thread)
// impl TryFrom<&str> for ParseTarget {
//     type Error = crate::error::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {

//     }
// }

#[cfg(test)]
mod ast_test {
    // use std::{fs::File, io::Write};

    use super::{
        comment::{offline::OfflineComment, position::OfflinePosition, Comments},
        ParseTarget,
    };

    #[test]
    fn parse_target() {
        let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        <style>
        .ui{
            height : fill;
            width : fill;
            show_bg : true;
        }
        </style>
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        let mut parse = ParseTarget::default();
        parse.set_template("<window class=\"ui\">\n            </window>\n        ");
        parse.set_script("let mut counter:usize = 0\n\n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        ");
        parse.set_style(".ui{\n            height : fill;\n            width : fill;\n            show_bg : true;\n        }\n        ");
        parse.set_comment(vec![OfflineComment::from((
            vec![Comments::File("This is a comment1")],
            OfflinePosition::AboveTemplate,
        ))]);
        assert_eq!(target, parse);
    }

    #[test]
    fn parse_empty() {
        let input = r#"
       
        "#;
        let target = ParseTarget::try_from(input);
        dbg!(target);
    }

    #[test]
    fn parse_only_code() {
        let input = r#"
        let a:&str = "trest";
        "#;
        let target = ParseTarget::try_from(input);
        dbg!(target);
    }

    #[test]
    fn display() {
        let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/rsx/parser/template.rsx")
        //         .unwrap();
        // let _ = f.write_all(target.to_string().as_bytes());
        dbg!(target.to_string());
    }

    #[test]
    fn display_no_template() {
        let input = r#"//! This is a comment1
        //! This is a comment2
        //! This is a comment3
        // This is line comment
        <template></template>
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0
        
        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>
        
        <style></style>
        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/rsx/parser/template.rsx")
        //         .unwrap();
        // let _ = f.write_all(target.to_string().as_bytes());
        assert_eq!(target.to_string().as_str(),"//! This is a comment1\n//! This is a comment2\n//! This is a comment3\n// This is line comment\n\n/// This is a doc comment\n/// hello\n<script>\nlet mut counter:usize = 0\n        \n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        \n</script>\n\n// This is line comment2\n// end of line comment\n");
    }

    #[test]
    fn display_only_comments() {
        let input = r#"//! This is a comment1
        //! This is a comment2
        //! This is a comment3
        // This is line comment
        
        /// This is a doc comment
        /// hello
        // This is line comment2
        // end of line comment
        "#;

        let target = ParseTarget::try_from(input).unwrap();
        dbg!(&target.to_string());
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/rsx/parser/template.rsx")
        //         .unwrap();
        // let _ = f.write_all(target.to_string().as_bytes());
    }
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
