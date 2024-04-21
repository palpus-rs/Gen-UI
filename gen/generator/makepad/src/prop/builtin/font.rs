use gen_utils::common::token_tree_ident;
use proc_macro2::TokenTree;
use syn::token;

use crate::prop::TEXT;

use super::{bind_prop, string_prop};

pub fn text(value: &str) -> (String, Vec<TokenTree>) {
    // string_prop(TEXT, value)
    (TEXT.to_string(), vec![token_tree_ident(value)])
}

pub fn text_bind(value: &str) -> Vec<TokenTree> {
    bind_prop(TEXT, value)
}