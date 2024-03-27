use std::{borrow::Cow, collections::HashMap};

use parser::Style;

use super::{ConvertStyle, MakepadConverter};

/// 平展样式
pub fn expand_style(s: &Box<Style>) -> Option<ConvertStyle> {
    let mut res = HashMap::new();
    // handle props
    if s.has_props() {
        let style_name = s.get_name();
        let props = s.get_props().unwrap();
        match s.get_type() {
            parser::StyleType::Class | parser::StyleType::Id => {
                res.insert(Cow::Borrowed(style_name), Cow::Borrowed(props))
            }
            parser::StyleType::Pseudo => {
                // find the parent and set maybe here need to do something special
                // so write todo to watch
                todo!("style pseudo");
            }
        };
    }
    // handle children
    if s.has_children() {
        for item in s.get_children().unwrap() {
            match MakepadConverter::convert_style(item) {
                Some(styles) => {
                    let _ = res.extend(styles);
                }
                None => {}
            };
        }
    }
    if res.is_empty() {
        return None;
    }
    Some(res)
}

/// expand all style sheet
pub fn handle_style(ast: &parser::ParseResult) -> Option<ConvertStyle> {
    let mut res = HashMap::new();
    for style in ast.style().unwrap() {
        match MakepadConverter::convert_style(style) {
            Some(styles) => res.extend(styles),
            None => return None,
        };
    }
    Some(res)
}