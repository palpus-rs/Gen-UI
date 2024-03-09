use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{prop_bg, prop_class, prop_clip_x, prop_clip_y, prop_height, prop_id, prop_line_spacing, prop_margin, prop_padding, prop_show_bg, prop_spacing, prop_width, PropRole},
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
        "height" => prop_height(prop_name, v),
        "width" => prop_width(prop_name, v),
        // match to `draw_bg`
        "background_color" => prop_bg(v),
        // match to `show_bg`
        "background_visible"=> prop_show_bg(v),
        "padding" => prop_padding(v),
        "margin" => prop_margin(v),
        "spacing" => prop_spacing(v),
        "line_spacing" => prop_line_spacing(v),
        "clip_x" => prop_clip_x(v),
        "clip_y" => prop_clip_y(v),
        _ => Err(Errors::unmatched_prop(prop_name, Widgets::Window)),
    }
}
