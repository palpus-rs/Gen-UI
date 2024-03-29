use parser::Value;

use crate::error::Errors;

use super::{
    align::prop_align,
    clip::{prop_clip_x, prop_clip_y},
    flow::prop_flow,
    link::{prop_class, prop_id},
    margin::prop_margin,
    padding::prop_padding,
    position::prop_abs_prop,
    prop_brightness, prop_color, prop_curve, prop_font, prop_font_size, prop_height_factor,
    prop_text_wrap, prop_top_drop,
    scroll::prop_scroll,
    size::{prop_height, prop_width},
    spacing::{prop_line_spacing, prop_spacing},
    PropRole,
};

/// Convert properties to Makepad::Walk
/// - height
/// - width
/// - absolute_position => abs_pos
/// - margin
pub fn prop_walk(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        // from Walk
        "height" => prop_height(v),
        // from Walk
        "width" => prop_width(v),
        // from Walk
        "absolute_position" => prop_abs_prop(v),
        // from Walk
        "margin" => prop_margin(v),
        _ => Err(Errors::UnAcceptConvertRange),
    }
}

/// Convert properties id can class
pub fn prop_link(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "id" => prop_id(v),
        "class" => prop_class(v),
        _ => Err(Errors::UnAcceptConvertRange),
    }
}

/// Convert properties to Makepad::Layout
/// - padding
/// - spacing
/// - line_spacing
/// - clip_x
/// - clip_y
/// - align
/// - flow
/// - scroll
pub fn prop_layout(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        // from Layout
        "padding" => prop_padding(v),
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
        _ => Err(Errors::UnAcceptConvertRange),
    }
}
/// Convert properties to Makepad::TextStyle
pub fn prop_draw_text(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "color" => prop_color(v),
        // from text_style
        "wrap" => prop_text_wrap(v),
        "font" => prop_font(v),
        "font_size" => prop_font_size(v),
        "brightness" => prop_brightness(v),
        "curve" => prop_curve(v),
        "line_spacing" => prop_line_spacing(v),
        "top_drop" => prop_top_drop(v),
        "height_factor" => prop_height_factor(v),
        _ => Err(Errors::UnAcceptConvertRange),
    }
}
