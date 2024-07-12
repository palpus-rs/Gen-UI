use std::{collections::HashMap, iter};

use gen_parser::{ASTNodes, PropertyKeyType, PropsKey, Style, StyleType, Value};
use gen_utils::props_manul;

use super::prop::ConvertStyle;

/// 平展样式
pub fn expand_style(style: &Box<Style>) -> Option<ConvertStyle> {
    let mut res: HashMap<String, HashMap<PropsKey, Value>> = HashMap::new();
    // handle props
    if style.has_props() {
        let style_name = style.get_name();
        let props = style.get_props().unwrap();
        match style.get_type() {
            StyleType::Class | StyleType::Id => {
                let _ = res.insert(style_name.to_string(), props.clone());
            }
            StyleType::Pseudo => {
                // todo!(这里暂时忽略掉了不是Animation手册中的属性，后续若有需要，还需继续改进)
                let style_name = style.get_parent().unwrap().get_name();

                let animation_props = props_manul::Animation::props();

                let prop = PropsKey::new(
                    format!("animation::{}", style.get_name()).as_str(),
                    true,
                    PropertyKeyType::Normal,
                );

                let animation_values = props
                    .iter()
                    .filter(|(k, _)| animation_props.contains(&k.name()))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<PropsKey, Value>>();

                let value = Value::Animation(animation_values);
                
                res.insert(style_name.to_string(), iter::once((prop, value)).collect());
            }
        };
    }
    // handle children
    if style.has_children() {
        let children = style.get_children().unwrap();
        match handle_styles(children) {
            Some(children_styles) => {
                // 查找是否有重复的key, 有则合并
                for (k, v) in children_styles {
                    match res.get_mut(&k) {
                        Some(res) => {
                            let _ = res.extend(v);
                        }
                        None => {
                            let _ = res.insert(k, v);
                        }
                    }
                }
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
pub fn handle_styles(styles: &Vec<ASTNodes>) -> Option<ConvertStyle> {
    let mut res:HashMap<String, HashMap<PropsKey, Value>> = HashMap::new();
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
