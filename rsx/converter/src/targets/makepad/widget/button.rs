use parser::{PropsKey, Value};

use crate::{error::Errors, targets::makepad::{prop_text, PropRole}};

use super::Widgets;

/// handle makepad button widget match
pub fn button(k:&PropsKey,v:&Value)->Result<PropRole,Errors>{
    let ty = k.ty();
    let prop_name = k.name();
    match ty {
        parser::PropertyKeyType::Normal => normal_button(prop_name,v),
        parser::PropertyKeyType::Bind => todo!(),
        parser::PropertyKeyType::Function => todo!(),
    }
}

fn normal_button(prop_name:&str,v:&Value)->Result<PropRole,Errors>{
    match prop_name {
        "text" => {
            prop_text(prop_name,v)
        },
        _ =>Err(Errors::unmatched_prop(prop_name, Widgets::Button)),
    }
}