use gen_parser::PropsKey;
use gen_utils::common::{token_stream_to_tree, token_tree_ident, trees_to_token_stream};
use proc_macro2::{Group, TokenStream, TokenTree};

use crate::{gen::FieldTable, utils::{if_group, self_event_react}};

pub fn event(
    root: Option<String>,
    id: String,
    pv: (PropsKey, String, TokenStream),
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    let (ep, ident, code) = pv;

    match ep.name() {
        "clicked" => button_clicked(root, id, ident, code, field_table),
        _ => panic!("not found event in button"),
    }
}

fn button_clicked(
    root: Option<String>,
    id: String,
    ident: String,
    code: TokenStream,
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    // 1. 获取field_table中的fields 并且遍历code中的节点，发现有field_table中的field则替换为field_table的prefix + field
    let prefix = token_stream_to_tree(field_table.get_prefix());
    let fields = field_table
        .get_fields()
        .iter()
        .filter(|item| {
            if let TokenTree::Ident(_) = item {
                return true;
            }
            false
        })
        .map(|item| {
            return if let TokenTree::Ident(ident) = item {
                ident.to_string()
            } else {
                panic!("field_table中的field必须是Ident类型{:#?}", item)
            };
        })
        .collect::<Vec<String>>();
    let mut code = token_stream_to_tree(code);
    let _ = visit_tree(&mut code, prefix, &fields);

    // 2. 调用self_event_react方法构造
    
    let mut tk = vec![token_tree_ident("if")];
    tk.extend(self_event_react(root, "button", &id, &ident, code));
    tk
}

fn visit_tree(tk: &mut Vec<TokenTree>, prefix: Vec<TokenTree>, fields: &Vec<String>) -> () {
    // 直接定位到Group中
    for (index, item) in tk.clone().iter().enumerate() {
        match item {
            TokenTree::Group(group) => visit_tree(
                &mut token_stream_to_tree(group.stream()),
                prefix.clone(),
                fields,
            ),
            TokenTree::Ident(ident) => {
                if fields.contains(&ident.to_string()) {
                    // 向当前索引位置插入prefix
                   if index > 0{
                    tk.splice((index - 1)..index, prefix.clone());
                   }else{
                    tk.splice(0..0, prefix.clone());
                   }
                }
            }
            _ => continue,
        }
    }
}
