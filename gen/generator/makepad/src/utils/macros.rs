use gen_utils::common::*;
use proc_macro2::TokenTree;

#[macro_export]
macro_rules! str_to_string_try_from {
    ($Target:ty) => {
        impl TryFrom<&String> for $Target {
            type Error = Errors;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }
    };
}



/// generate `Live, LiveHook,`
pub fn derive_live_livehook() -> Vec<TokenTree> {
    vec![
        token_tree_ident("Live"),
        token_tree_punct_alone(','),
        token_tree_ident("LiveHook"),
        token_tree_punct_alone(','),
    ]
}

pub fn derive_default_none() -> Vec<TokenTree> {
    vec![token_tree_ident("DefaultNone"), token_tree_punct_alone(',')]
}

/// generate `live_design!{...}`
pub fn live_design_macro(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("live_design"),
        token_tree_punct_alone('!'),
        token_tree_group(code),
    ]
}

/// 按需添加derive宏
/// `#[derive([Debug, Clone, Default])]`
pub fn derive_macros(marcos: Vec<&str>) -> Vec<TokenTree> {
    // let len  = marcos.len();
    let mut marcos_tks = Vec::new();
    marcos.iter().enumerate().for_each(|(i, v)| {
        marcos_tks.push(token_tree_ident(v));
        if i != marcos.len() {
            marcos_tks.push(token_tree_punct_alone(','))
        }
    });
    vec![
        token_tree_punct_alone('#'),
        token_tree_group_bracket(vec![
            token_tree_ident("derive"),
            token_tree_group_paren(marcos_tks),
        ]),
    ]
}

pub fn id_macro(id: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident("id"),
        token_tree_punct_alone('!'),
        token_tree_group_paren(vec![token_tree_ident(id)]),
    ]
}

pub fn live_attr() -> Vec<TokenTree> {
    vec![
        token_tree_punct_joint('#'),
        token_tree_group_bracket(vec![token_tree_ident("live")]),
    ]
}
pub fn rust_attr() -> Vec<TokenTree> {
    vec![
        token_tree_punct_joint('#'),
        token_tree_group_bracket(vec![token_tree_ident("rust")]),
    ]
}

/// generate `#[...]`
pub fn derive_marco_wrap(target: &str)-> Vec<TokenTree> {
    vec![
        token_tree_punct_joint('#'),
        token_tree_group_bracket(vec![token_tree_ident(target)]),
    ]
}