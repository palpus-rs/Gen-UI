use gen_utils::common::{token_tree_group_paren, token_tree_ident, token_tree_punct_alone};
use proc_macro2::TokenTree;

mod bg;
mod font;
mod layout;
mod walk;
mod shader;
mod base;

pub use bg::*;
pub use font::*;
pub use layout::*;
use quote::quote;
pub use walk::*;
pub use base::*;
pub use shader::*;

pub fn easy_prop(prop: &str, value: &str) -> (String, Vec<TokenTree>) {
    (prop.to_string(), vec![token_tree_ident(value)])
}

pub fn normal_prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident(prop_name),
        token_tree_punct_alone(':'),
        token_tree_ident(value),
        token_tree_punct_alone(','),
    ]
}

pub fn string_prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    let value = quote! {#value};
    let mut tk = vec![token_tree_ident(prop_name), token_tree_punct_alone(':')];
    tk.extend(value);

    tk.push(token_tree_punct_alone(','));
    tk
}

pub fn bind_prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident(prop_name),
        token_tree_punct_alone(':'),
        token_tree_group_paren(vec![
            token_tree_ident("self"),
            token_tree_punct_alone('.'),
            token_tree_ident("instance"),
            token_tree_punct_alone('.'),
            token_tree_ident(value),
        ]),
        token_tree_punct_alone(','),
    ]
}
