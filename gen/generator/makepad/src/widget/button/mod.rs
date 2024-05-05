mod prop;
mod prop_ptr;

pub use prop::ButtonProps;
pub use prop_ptr::ButtonPropPtr;
use gen_parser::PropsKey;
use gen_utils::common::{
    camel_to_snake, token_stream_to_tree, token_tree_group_paren, token_tree_ident,
    token_tree_punct_alone, token_tree_punct_joint, trees_to_token_stream,
};
use proc_macro2::{Group, TokenStream, TokenTree};

use crate::{
    
    prop::{
        builtin::{text, text_bind},
        TEXT,
    },
    utils::{apply_over_and_redraw, self_event_react},
    widget::prop_ignore,
};

pub fn prop(prop_name: &str, value: &str) -> (String, Vec<TokenTree>) {
    match prop_name {
        TEXT => text(value),
        _ => {
            if !prop_ignore(prop_name) {
                panic!("cannot match prop");
            }
            todo!()
        }
    }
}

pub fn prop_token(prop_name: &str, value: &str) -> (Vec<TokenTree>, TokenTree) {
    match prop_name {
        TEXT => (text_bind(value), token_tree_ident("String")),
        _ => todo!(),
    }
}

// pub fn event(
//     root: Option<String>,
//     id: String,
//     pv: (PropsKey, String, TokenStream),
//     field_table: &FieldTable,
// ) -> Vec<TokenTree> {
//     let (ep, call, code) = pv;

//     match ep.name() {
//         "clicked" => button_clicked(root, id, &call, "clicked", code, field_table),
//         _ => panic!("not found event in button"),
//     }
// }

// fn button_clicked(
//     root: Option<String>,
//     id: String,
//     call: &str,
//     ident: &str,
//     code: TokenStream,
//     field_table: &FieldTable,
// ) -> Vec<TokenTree> {
//     // 1. 获取field_table中的fields 并且遍历code中的节点，发现有field_table中的field则替换为field_table的prefix + field
//     let prefix = field_table.self_prefix();

//     let fields = field_table.to_field_strs();

//     let visitor = EventVisitor::new(prefix, fields);

//     let (mut code, updates) = visitor.visit(token_stream_to_tree(code));
//     // 添加调用方法
//     // 后续需要修改来支持参数传入
//     code.extend(vec![
//         token_tree_ident(call),
//         token_tree_punct_joint('('),
//         token_tree_punct_joint(')'),
//         token_tree_punct_alone(';'),
//     ]);

//     // 完成调用后，再次进行渲染
//     let field_items = field_table.get_fields();
//     // 遍历update进行更新
//     let update_tk = updates.iter().fold(vec![], |mut acc, update| {
//         acc.extend(match_update(
//             root.clone(),
//             field_items
//                 .iter()
//                 .find(|item| item.value.eq(update))
//                 .unwrap(),
//         ));
//         acc
//     });

//     // 2. 调用self_event_react方法构造

//     let mut tk = vec![token_tree_ident("if")];
//     tk.extend(self_event_react(root, "button", &id, ident, code));
//     tk.extend(update_tk);
//     tk
// }

struct EventVisitor {
    replace: TokenStream,
    fields: Vec<String>,
}

impl EventVisitor {
    pub fn new(replace: TokenStream, fields: Vec<String>) -> Self {
        Self { replace, fields }
    }
    fn visit(&self, target: Vec<TokenTree>) -> (Vec<TokenTree>, Vec<String>) {
        let mut res = target.clone();
        let mut indexs = Vec::new();
        // 需要更新的props
        let mut updates = Vec::new();
        target.iter().enumerate().for_each(|(index, item)| {
            if let TokenTree::Group(group) = item {
                let (handled, c_updates) = self.visit(token_stream_to_tree(group.stream()));
                updates.extend(c_updates);
                res[index] = TokenTree::Group(Group::new(
                    group.delimiter(),
                    trees_to_token_stream(handled),
                ));
            }
            if let TokenTree::Ident(ident) = item {
                if self.fields.contains(&ident.to_string()) {
                    // 收集需要更改的索引
                    indexs.push(index);
                }
            }
        });
        for index in indexs.iter().rev() {
            updates.push(res[*index].to_string());
            res.splice(*index..*index, self.replace.clone());
        }

        (res, updates)
    }
}

// fn match_update(root: Option<String>, target: &FieldItem) -> Vec<TokenTree> {
//     let FieldItem {
//         source,
//         prop,
//         value,
//         id,
//     } = target;

//     let mut live_tk = vec![token_tree_ident(prop), token_tree_punct_alone(':')];

//     live_tk.push(token_tree_group_paren(if root.is_some() {
//         vec![
//             token_tree_ident("self"),
//             token_tree_punct_joint('.'),
//             token_tree_ident("instance"),
//             token_tree_punct_joint('.'),
//             token_tree_ident(value),
//         ]
//     } else {
//         vec![
//             token_tree_ident("self"),
//             token_tree_punct_joint('.'),
//             token_tree_ident(value),
//         ]
//     }));

//     let tag = camel_to_snake(&source.to_string());
//     apply_over_and_redraw(root, tag, id, live_tk)
// }
