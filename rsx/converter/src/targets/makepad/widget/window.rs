use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{prop_id, PropRole},
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
        "class" => todo!(),
        _ => Err(Errors::unmatched_prop(prop_name, Widgets::Window)),
    }
}
