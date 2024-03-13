use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{
        prop_bg, prop_block_signal_event, prop_cursor, prop_event_order, prop_grab_key_focus,
        prop_layout, prop_link, prop_show_bg, prop_view_optimize, prop_visible, prop_walk,
        PropRole,
    },
};

use super::Widgets;

pub fn window(k: &PropsKey, v: &Value) -> Result<PropRole, Errors> {
    // let ty = k.ty();
    let prop_name = k.name();
    normal_window(prop_name, v)
    // match ty {
    //     parser::PropertyKeyType::Normal => normal_window(prop_name, v),
    //     parser::PropertyKeyType::Bind => ,
    //     parser::PropertyKeyType::Function => todo!(),
    // }
}

fn normal_window(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        // match to `draw_bg`
        "background_color" => prop_bg(v),
        // match to `show_bg`
        "background_visible" => prop_show_bg(v),
        // view private properties
        "optimize" => prop_view_optimize(v),
        "event_order" => prop_event_order(v),
        "visible" => prop_visible(v),
        "grab_key_focus" => prop_grab_key_focus(v),
        "block_signal_event" => prop_block_signal_event(v),
        "cursor" => prop_cursor(v),
        // "scroll_bars"=> prop_scroll_bars(v),
        _ => prop_link(prop_name, v)
            .or_else(|_| prop_walk(prop_name, v))
            .or_else(|_| prop_layout(prop_name, v))
            .or_else(|_| Err(Errors::unmatched_prop(prop_name, Widgets::View))),
    }
}
