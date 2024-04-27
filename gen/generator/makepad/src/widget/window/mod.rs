mod prop;

pub use prop::WindowProps;

use proc_macro2::TokenTree;

use super::view;

pub fn prop(prop_name: &str, value: &str) -> (String, Vec<TokenTree>) {
    view::prop(prop_name, value)
}
