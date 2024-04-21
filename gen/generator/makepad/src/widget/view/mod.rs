use std::collections::HashMap;

use gen_utils::common::token_tree_ident;
use proc_macro2::{TokenStream, TokenTree};

use crate::prop::{
    builtin::{align, draw_bg, height, show_bg, show_bg_bind, width}, ALIGN, DRAW_BG, HEIGHT, SHOW_BG, WIDTH
};

use super::prop_ignore;

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

    // match prop_name {
    //     // match to `draw_bg`
    //     "draw_bg" => prop_bg(v),
    //     // match to `show_bg`
    //     "show_bg" => prop_show_bg(v),
    //     // view private properties
    //     "optimize" => prop_view_optimize(v),
    //     "event_order" => prop_event_order(v),
    //     "visible" => prop_visible(v),
    //     "grab_key_focus" => prop_grab_key_focus(v),
    //     "block_signal_event" => prop_block_signal_event(v),
    //     "cursor" => prop_cursor(v),
    //     // "scroll_bars"=> prop_scroll_bars(v),
    //     _ => prop_link(prop_name, v)
    //         .or_else(|_| prop_walk(prop_name, v))
    //         .or_else(|_| prop_layout(prop_name, v))
    //         .map_err(Into::into),
    // }
}

/// return prop token and prop type token
/// (prop_tk, type_tk)
pub fn prop_token(prop_name: &str, value: &str) -> (Vec<TokenTree>, TokenTree) {
    match prop_name {
        SHOW_BG => (show_bg_bind(value), token_tree_ident("bool")),
        _ => todo!(),
    }
}
