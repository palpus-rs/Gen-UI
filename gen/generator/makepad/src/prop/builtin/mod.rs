use gen_utils::common::{token_tree_group_paren, token_tree_ident, token_tree_punct_alone};
use proc_macro2::TokenTree;

mod bg;
mod walk;
mod layout;

pub use bg::*;
pub use walk::*;
pub use layout::*;

pub fn normal_prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident(prop_name),
        token_tree_punct_alone(':'),
        token_tree_ident(value),
        token_tree_punct_alone(','),
    ]
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
