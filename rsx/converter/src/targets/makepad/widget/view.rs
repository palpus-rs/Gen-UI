use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{prop_bg, prop_class, prop_height, prop_id, prop_margin, prop_padding, prop_show_bg, prop_spacing, prop_width, PropRole},
};

use super::Widgets;

pub fn view(k: &PropsKey, v: &Value) -> Result<PropRole, Errors> {
    let ty = k.ty();
    let prop_name = k.name();
    match ty {
        parser::PropertyKeyType::Normal => normal_view(prop_name, v),
        parser::PropertyKeyType::Bind => todo!(),
        parser::PropertyKeyType::Function => todo!(),
    }
}

fn normal_view(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
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
        _ => Err(Errors::unmatched_prop(prop_name, Widgets::Window)),
    }
}