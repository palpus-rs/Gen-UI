use gen_utils::common::token_tree_ident;
use proc_macro2::TokenTree;

use crate::prop::{HEIGHT, WIDTH};

use super::normal_prop;

pub fn height(value: &str) -> (String, Vec<TokenTree>) {
    // normal_prop(HEIGHT, value)
    (HEIGHT.to_string(), vec![token_tree_ident(value)])
}
pub fn width(value: &str) -> (String, Vec<TokenTree>) {
    // normal_prop(WIDTH, value)
    (WIDTH.to_string(), vec![token_tree_ident(value)])
}