use gen_utils::common::{token_tree_ident, token_tree_punct_alone,token_tree_group_paren};
use proc_macro2::{TokenStream, TokenTree};

mod bg;

pub use bg::*;


pub fn normal_prop(prop_name: &str, value: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident(prop_name),
        token_tree_punct_alone(':'),
        token_tree_ident(value),
        token_tree_punct_alone(',')
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
        token_tree_punct_alone(',')
    ]
    
}