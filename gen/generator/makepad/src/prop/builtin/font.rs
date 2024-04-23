use gen_utils::common::token_tree_ident;
use proc_macro2::TokenTree;
use syn::token;

use crate::prop::{DRAW_DEPTH, FONT_SCALE, TEXT};

use super::{bind_prop, easy_prop, string_prop};

pub fn text(value: &str) -> (String, Vec<TokenTree>) {
    // string_prop(TEXT, value)
    easy_prop(TEXT, value)
}

pub fn text_bind(value: &str) -> Vec<TokenTree> {
    bind_prop(TEXT, value)
}

pub fn font_scale(value: &str) -> (String, Vec<TokenTree>) {
    easy_prop(FONT_SCALE, value)
}

pub fn draw_depth(value: &str) -> (String, Vec<TokenTree>) {
    easy_prop(DRAW_DEPTH, value)
}

