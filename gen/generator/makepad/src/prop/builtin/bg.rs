use gen_utils::common::{
    token_stream_to_tree, token_tree_group, token_tree_ident, token_tree_punct_alone,
};
use proc_macro2::TokenTree;
use quote::quote;

use crate::prop::{DRAW_BG, SHOW_BG};

use super::bind_prop;

pub fn show_bg(value: &str) -> (String, Vec<TokenTree>) {
    // normal_prop(SHOW_BG, value)
    (SHOW_BG.to_string(), vec![token_tree_ident(value)])
}

pub fn show_bg_bind(value: &str) -> Vec<TokenTree> {
    bind_prop(SHOW_BG, value)
}

pub fn draw_bg(value: &str) -> (String, Vec<TokenTree>) {
    let color = token_stream_to_tree(quote! { #value });
    let mut color_tk = vec![token_tree_ident("color"), token_tree_punct_alone(':')];
    color_tk.extend(color);

    (DRAW_BG.to_string(), vec![token_tree_group(color_tk)])
}
