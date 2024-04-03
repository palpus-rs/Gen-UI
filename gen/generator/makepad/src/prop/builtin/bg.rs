use proc_macro2::TokenTree;

use crate::prop::SHOW_BG;

use super::normal_prop;

pub fn show_bg(value: &str) -> Vec<TokenTree> {
    normal_prop(SHOW_BG, value)
}
