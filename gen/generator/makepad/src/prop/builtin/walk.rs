use proc_macro2::TokenTree;

use crate::prop::{HEIGHT, WIDTH};

use super::normal_prop;

pub fn height(value: &str) -> Vec<TokenTree> {
    normal_prop(HEIGHT, value)
}
pub fn width(value: &str) -> Vec<TokenTree> {
    normal_prop(WIDTH, value)
}