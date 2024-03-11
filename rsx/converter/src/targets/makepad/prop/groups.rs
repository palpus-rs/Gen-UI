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
