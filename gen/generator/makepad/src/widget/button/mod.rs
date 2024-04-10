use gen_parser::PropsKey;
use gen_utils::common::{
    token_stream_to_tree, token_tree_ident, token_tree_punct_alone, token_tree_punct_joint,
    trees_to_token_stream,
};
use proc_macro2::{Group, TokenStream, TokenTree};

use crate::{
    gen::FieldTable,
    prop::{
        builtin::{text, text_bind},
        TEXT,
    },
    utils::self_event_react,
    widget::prop_ignore,
};

pub fn prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    match prop_name {
        TEXT => text(value),
        _ => {
            if !prop_ignore(prop_name) {
                panic!("cannot match prop");
            }
            vec![]
        }
    }
}

pub fn prop_token(prop_name: &str, value: &str) -> (Vec<TokenTree>, TokenTree) {
    match prop_name {
        TEXT => (text_bind(value), token_tree_ident("String")),
        _ => todo!(),
    }
}

pub fn event(
    root: Option<String>,
    id: String,
    pv: (PropsKey, String, TokenStream),
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    let (ep, call, code) = pv;

    match ep.name() {
        "clicked" => button_clicked(root, id, &call, "clicked", code, field_table),
        _ => panic!("not found event in button"),
    }
}

fn button_clicked(
    root: Option<String>,
    id: String,
    call: &str,
    ident: &str,
    code: TokenStream,
    field_table: &FieldTable,
) -> Vec<TokenTree> {
    // 1. 获取field_table中的fields 并且遍历code中的节点，发现有field_table中的field则替换为field_table的prefix + field
    let prefix = field_table.self_prefix();
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
    let visitor = EventVisitor::new(prefix, fields);

    let mut code = visitor.visit(token_stream_to_tree(code));
    // 添加调用方法
    // 后续需要修改来支持参数传入
    code.extend(vec![
        token_tree_ident(call),
        token_tree_punct_joint('('),
        token_tree_punct_joint(')'),
        token_tree_punct_alone(';'),
    ]);

    // 2. 调用self_event_react方法构造

    let mut tk = vec![token_tree_ident("if")];
    tk.extend(self_event_react(root, "button", &id, ident, code));
    tk
}

struct EventVisitor {
    replace: TokenStream,
    fields: Vec<String>,
}

impl EventVisitor {
    pub fn new(replace: TokenStream, fields: Vec<String>) -> Self {
        Self { replace, fields }
    }
    fn visit(&self, target: Vec<TokenTree>) -> Vec<TokenTree> {
        let mut res = target.clone();
        let mut indexs = Vec::new();
        target.iter().enumerate().for_each(|(index, item)| {
            if let TokenTree::Group(group) = item {
                let handled = self.visit(token_stream_to_tree(group.stream()));
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
            res.splice(*index..*index, self.replace.clone());
        }
        res
    }
}
