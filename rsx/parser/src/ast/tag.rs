use std::fmt::Display;

use crate::{END_SIGN, END_START_SIGN, SELF_END_SIGN, TAG_START};

use super::{props_to_string, ASTNodes, Props};

#[derive(Debug, Clone, PartialEq)]
pub enum CloseType {
    /// <xxx /> -> `/>`
    SelfClosed,
    /// <xxx></xxx> -> `>`
    Normal,
}

#[allow(dead_code)]
impl CloseType {
    pub fn is_normal(&self) -> bool {
        matches!(self, CloseType::Normal)
    }
    pub fn is_self_close(&self) -> bool {
        !self.is_normal()
    }
}

impl Default for CloseType {
    fn default() -> Self {
        CloseType::Normal
    }
}

impl Display for CloseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            CloseType::SelfClosed => SELF_END_SIGN,
            CloseType::Normal => END_SIGN,
        };
        f.write_str(res)
    }
}

impl From<&str> for CloseType {
    fn from(value: &str) -> Self {
        match value {
            SELF_END_SIGN => CloseType::SelfClosed,
            END_SIGN => CloseType::Normal,
            _ => panic!("Invalid close type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag<'a> {
    name: &'a str,
    ty: CloseType,
    props: Props<'a>,
    children: Option<Vec<ASTNodes<'a>>>,
    parent: Option<ASTNodes<'a>>,
}

#[allow(dead_code)]
impl<'a> Tag<'a> {
    pub fn new(
        name: &'a str,
        props: Props<'a>,
        ty: CloseType,
        children: Option<Vec<ASTNodes<'a>>>,
        parent: Option<ASTNodes<'a>>,
    ) -> Self {
        Tag {
            name,
            ty,
            props,
            children,
            parent,
        }
    }
    pub fn new_tag_start(name: &'a str) -> Self {
        Self {
            name,
            ty: Default::default(),
            props: None,
            children: None,
            parent: None,
        }
    }
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
    pub fn set_ty(&mut self, ty: CloseType) {
        self.ty = ty;
    }
    pub fn set_props(&mut self, props: Props<'a>) {
        self.props = props;
    }
    pub fn set_children(&mut self, children: Vec<ASTNodes<'a>>) {
        match self.children {
            Some(_) => {
                let _ = self.children.replace(children);
            }
            None => self.children = Some(children),
        }
    }
    pub fn set_parent(&mut self, parent: ASTNodes<'a>) {
        match self.parent {
            Some(_) => {
                let _ = self.parent.replace(parent);
            }
            None => self.parent = Some(parent),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> CloseType {
        self.ty.clone()
    }
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
}

impl<'a> Display for Tag<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("{}{}", TAG_START, self.get_name(),));

        let props_str = props_to_string(self.props.clone());
        if !props_str.is_empty() {
            let _ = f.write_fmt(format_args!(" {}", props_str));
        }
        match self.ty {
            CloseType::SelfClosed => f.write_str(" />"),
            CloseType::Normal => {
                let _ = f.write_str(END_SIGN);
                // add children
                if self.has_children() {
                    let _ = f.write_fmt(format_args!(
                        "\n{}",
                        self.children
                            .as_ref()
                            .unwrap()
                            .into_iter()
                            .map(|item| item.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    ));
                    let _ = f.write_str("\n");
                }
                f.write_fmt(format_args!(
                    "{}{}{}",
                    END_START_SIGN,
                    self.get_name(),
                    END_SIGN
                ))
            }
        }
    }
}

// impl From<(&str,Props ,CloseType)> for Tag {
//     fn from(value: (&str, Props,CloseType)) -> Self {
//         Tag::new(value.0, value.1,value.2)
//     }
// }

#[cfg(test)]
mod test_tag {
    use super::{CloseType, Tag};
    use std::collections::HashMap;

    #[test]
    fn get_name() {
        let tag_normal = Tag::new("input", None, CloseType::Normal, None, None);
        let tag_close_self = Tag::new("select", None, CloseType::SelfClosed, None, None);

        assert_eq!(tag_normal.get_name(), "input");
        assert_eq!(tag_close_self.get_name(), "select");
    }
    #[test]
    fn get_type() {
        let tag_normal = Tag::new("input", None, CloseType::Normal, None, None);
        let tag_close_self = Tag::new("select", None, CloseType::SelfClosed, None, None);

        assert_eq!(tag_normal.get_type(), CloseType::Normal);
        assert_eq!(tag_close_self.get_type(), CloseType::SelfClosed);
    }
    #[test]
    fn display() {
        let tag_normal = Tag::new("input", None, CloseType::Normal, None, None);
        let tag_close_self = Tag::new("select", None, CloseType::SelfClosed, None, None);

        assert_eq!(tag_normal.to_string().as_str(), "<input>");
        assert_eq!(tag_close_self.to_string().as_str(), "<select/>");
    }

    #[test]
    fn display_complex() {
        let mut props = HashMap::new();
        props.insert("name", "MyInput".into());
        props.insert("value", "17".into());
        props.insert("placeholder", "please enter ...".into());
        let tag_normal = Tag::new("input", Some(props), CloseType::Normal, None, None);
        let tag_close_self = Tag::new("select", None, CloseType::SelfClosed, None, None);
        // "<input name=\"MyInput\" value=\"17\" placeholder=\"please enter ...\" ><inpit>"
        // dbg!(tag_normal.to_string());
        assert_eq!(tag_close_self.to_string().as_str(), "<select />");
    }
}
