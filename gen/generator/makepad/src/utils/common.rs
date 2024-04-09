use gen_utils::common::*;
use proc_macro2::{TokenStream, TokenTree};

use super::{derive_macros, id_macro};

/// generate `live!{ //.. }`
pub fn live_macro(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("live"),
        token_tree_punct_alone('!'),
        token_tree_group(code),
    ]
}

/// generate `self.[ui_name].[tag_name].apply_over_and_redraw(cx, live!{...});`
pub fn apply_over_and_redraw(
    ui: Option<String>,
    tag: String,
    id: String,
    pv: Vec<TokenTree>,
) -> Vec<TokenTree> {
    let mut f = vec![token_tree_ident("self"), token_tree_punct_joint('.')];
    if ui.is_some() {
        f.push(token_tree_ident(ui.unwrap().as_str()));
        f.push(token_tree_punct_alone('.'));
    }

    f.extend(vec![
        token_tree_ident(&tag),
        token_tree_group_paren(vec![
            token_tree_ident("id"),
            token_tree_punct_alone('!'),
            token_tree_group_paren(vec![token_tree_ident(&id)]),
           
        ]),
        token_tree_punct_alone('.'),
        token_tree_ident("apply_over_and_redraw"),
        token_tree_group_paren(vec![
            token_tree_ident("cx"),
            token_tree_punct_alone(','),
            token_tree_ident("live"),
            token_tree_punct_alone('!'),
            token_tree_group(pv),
        ]),
        token_tree_punct_alone(';'),
    ]);
    f
}

/// ```
/// #[derive(Debug, Clone, Default)]
/// struct Instance {
///   pub field: Type,
/// }
/// ```
pub fn instance(kvs: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut tk = derive_macros(vec!["Debug", "Clone", "Default"]);
    tk.extend(vec![
        token_tree_ident("struct"),
        token_tree_ident("Instance"),
        token_tree_group(kvs),
    ]);
    tk
}

/// generate `impl xxx{...}`
pub fn impl_target(target: &str, code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("impl"),
        token_tree_ident(target),
        token_tree_group(code),
    ]
}

/// generate `fn new()->Self{}`
pub fn instance_new_fn(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("fn"),
        token_tree_ident("new"),
        token_tree_group_paren(vec![]),
        token_tree_punct_joint('-'),
        token_tree_punct_joint('>'),
        token_tree_ident("Self"),
        token_tree_group(code),
    ]
}

/// generate `...init code...; Self{...}`
pub fn instance_return_self(mut init: Vec<TokenTree>, code: Vec<TokenTree>) -> Vec<TokenTree> {
    init.extend(vec![token_tree_ident("Self"), token_tree_group(code)]);
    init
}

/// generate `pub field: Type,`
pub fn struct_field_type(field: &str, ty: TokenTree) -> Vec<TokenTree> {
    vec![
        token_tree_ident("pub"),
        token_tree_ident(field),
        token_tree_punct_alone(':'),
        ty,
        token_tree_punct_alone(','),
    ]
}

pub fn if_group(condition:Vec<TokenTree> ,code: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut if_tk = vec![
        token_tree_ident("if"),
    ];
    if_tk.extend(condition);
    if_tk.push(
        token_tree_group(code)
    );
    if_tk
}

/// generate `self.[ui_name].tag_name(id_macro!(id)).event_name(&actions){...}` 
pub fn self_event_react(ui:Option<String>, tag: &str, id:&str, event:&str, code:Vec<TokenTree>)->Vec<TokenTree>{
    let mut tk = vec![
        token_tree_ident("self"),
    ];

    if ui.is_some(){
        tk.push(token_tree_punct_alone('.'));
        tk.push(token_tree_ident(ui.unwrap().as_str()));
    }

    tk.extend(vec![
        token_tree_punct_alone('.'),
        token_tree_ident(tag),
        token_tree_group_paren(id_macro(id)),
        token_tree_punct_alone('.'),
        token_tree_ident(event),
        token_tree_group_paren(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("actions"),
        ]),
        token_tree_group(code),
    ]);

    tk
}