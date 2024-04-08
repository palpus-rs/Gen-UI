use proc_macro2::{TokenStream, TokenTree};

use crate::prop::SHOW_BG;

use super::{normal_prop, bind_prop};

pub fn show_bg(value: &str) -> Vec<TokenTree> {
    normal_prop(SHOW_BG, value)
}

pub fn show_bg_bind(value: &str) -> Vec<TokenTree> {
    bind_prop(SHOW_BG, value)
}
