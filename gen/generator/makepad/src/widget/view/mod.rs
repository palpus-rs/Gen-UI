use proc_macro2::TokenTree;

use crate::prop::{builtin::show_bg, DRAW_BG, SHOW_BG};

/// generate view widget prop
pub fn prop(prop_name: &str, value: &str)->Vec<TokenTree>{

    match prop_name{
        SHOW_BG => show_bg(value),
        _ => todo!()
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