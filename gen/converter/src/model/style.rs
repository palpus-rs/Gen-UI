use std::{borrow::Cow, collections::HashMap};

use gen_parser::{ASTNodes, Style, StyleType};

use super::prop::ConvertProp;

/// 平展样式
pub fn expand_style(style: &Box<Style>) -> Option<ConvertProp> {
    let mut res = HashMap::new();
    // handle props
    if style.has_props() {
        let style_name = style.get_name();
        let props = style.get_props().unwrap();
        match style.get_type() {
            StyleType::Class | StyleType::Id => {
                res.insert(style_name.to_string(), props.clone())
            }
            StyleType::Pseudo => {
                // find the parent and set maybe here need to do something special
                // so write todo to watch
                todo!("style pseudo");
            }
        };
    }
    // handle children
    if style.has_children() {
        let children = style.get_children().unwrap();
        match handle_styles(children) {
            Some(children_styles) => {
                let _ = res.extend(children_styles);
            }
            None => {}
        }
    }
    if res.is_empty() {
        return None;
    }
    Some(res)
}

/// expand all style sheet
pub fn handle_styles(styles: &Vec<ASTNodes>) -> Option<ConvertProp> {
    let mut res = HashMap::new();
    for style in styles {
        match style {
            ASTNodes::Style(style) => match expand_style(style) {
                Some(expanded_style) => {
                    let _ = res.extend(expanded_style);
                }
                None => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }
    }
    Some(res)
}
