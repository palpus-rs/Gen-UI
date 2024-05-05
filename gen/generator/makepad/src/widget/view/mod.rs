mod prop;
mod prop_ptr;
mod r#trait;

pub use prop::ViewProps;
pub use prop_ptr::ViewPropPtr;
pub use r#trait::*;

use gen_utils::common::token_tree_ident;
use proc_macro2::{TokenStream, TokenTree};

use crate::prop::{
    builtin::{align, draw_bg, height, show_bg, show_bg_bind, width}, ALIGN, DRAW_BG, HEIGHT, SHOW_BG, WIDTH
};



use super::{prop_ignore, StaticProps};


/// generate view widget prop
pub fn prop(prop_name: &str, value: &str) ->  (String, Vec<TokenTree>) {
    match prop_name {
        SHOW_BG => show_bg(value),
        DRAW_BG => draw_bg(value),
        HEIGHT => height(value),
        WIDTH => width(value),
        ALIGN => align(value),
        _ => {
            if !prop_ignore(prop_name) {
                panic!("cannot match prop");
            }
            todo!("unslolved prop")
        }
    }


}

/// return prop token and prop type token
/// (prop_tk, type_tk)
pub fn prop_token(prop_name: &str, value: &str) -> (Vec<TokenTree>, TokenTree) {
    match prop_name {
        SHOW_BG => (show_bg_bind(value), token_tree_ident("bool")),
        _ => todo!(),
    }
}
