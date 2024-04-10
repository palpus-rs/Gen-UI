use proc_macro2::TokenTree;

use crate::prop::TEXT;

use super::{bind_prop, string_prop};

pub fn text(value: &str) -> Vec<TokenTree> {
    string_prop(TEXT, value)
}

pub fn text_bind(value: &str) -> Vec<TokenTree> {
    bind_prop(TEXT, value)
}