use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{
        prop_abs_prop, prop_align, prop_bg, prop_block_signal_event, prop_class, prop_clip_x,
        prop_clip_y, prop_cursor, prop_event_order, prop_flow, prop_grab_key_focus, prop_height,
        prop_id, prop_line_spacing, prop_margin, prop_padding, prop_scroll, prop_show_bg,
        prop_spacing, prop_view_optimize, prop_visible, prop_width, PropRole,
    },
};

use super::Widgets;

pub fn window(k: &PropsKey, v: &Value) -> Result<PropRole, Errors> {
    let ty = k.ty();
    let prop_name = k.name();
    match ty {
        parser::PropertyKeyType::Normal => normal_window(prop_name, v),
        parser::PropertyKeyType::Bind => todo!(),
        parser::PropertyKeyType::Function => todo!(),
    }
}

fn normal_window(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "id" => prop_id(v),
        "class" => prop_class(v),
        // from Walk
        "height" => prop_height(prop_name, v),
        // from Walk
        "width" => prop_width(prop_name, v),
        // from Walk
        "absolute_position" => prop_abs_prop(v),
        // match to `draw_bg`
        "background_color" => prop_bg(v),
        // match to `show_bg`
        "background_visible" => prop_show_bg(v),
        // from Layout
        "padding" => prop_padding(v),
        // from Walk
        "margin" => prop_margin(v),
        // from Layout
        "spacing" => prop_spacing(v),
        // from Layout
        "line_spacing" => prop_line_spacing(v),
        // from Layout
        "clip_x" => prop_clip_x(v),
        // from Layout
        "clip_y" => prop_clip_y(v),
        // from Layout
        // "align_x" => prop_align_x(v),
        // "align_y" => prop_align_y(v),
        // template do not use align_x or align_y
        "align" => prop_align(v),
        // from Layout
        "flow" => prop_flow(v),
        // from Layout
        "scroll" => prop_scroll(v),
        // view private properties
        "optimize" => prop_view_optimize(v),
        "event_order" => prop_event_order(v),
        "visible" => prop_visible(v),
        "grab_key_focus" => prop_grab_key_focus(v),
        "block_signal_event" => prop_block_signal_event(v),
        "cursor" => prop_cursor(v),
        // "scroll_bars"=> prop_scroll_bars(v),
        _ => Err(Errors::unmatched_prop(prop_name, Widgets::Window)),
    }
}
