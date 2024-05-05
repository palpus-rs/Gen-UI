mod prop;
mod prop_ptr;
pub use prop::WindowProps;
pub use prop_ptr::WindowPropPtr;

use proc_macro2::TokenTree;

use super::view;

pub fn prop(prop_name: &str, value: &str) -> (String, Vec<TokenTree>) {
    view::prop(prop_name, value)
}
