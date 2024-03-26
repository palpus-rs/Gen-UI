use parser::Value;

use crate::{
    error::Errors,
    targets::makepad::{
        action::action_clicked, prop_layout, prop_link, prop_text, prop_walk, PropRole,
    },
};

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum ButtonAction {
    None,
    Clicked,
    Pressed,
    Released,
}

impl ButtonAction {
    pub fn match_action(action:&str)->ButtonAction{
        match action {
            "clicked" => ButtonAction::Clicked,
            "pressed" => ButtonAction::Pressed,
            "released" => ButtonAction::Released,
            _ => ButtonAction::None,
        }
    }
}

impl ToString for ButtonAction {
    fn to_string(&self) -> String {
        match self {
            ButtonAction::None => "ButtonAction::None",
            ButtonAction::Clicked => "ButtonAction::Clicked",
            ButtonAction::Pressed => "ButtonAction::Pressed",
            ButtonAction::Released => "ButtonAction::Released",
        }.to_string()
    }
}

/// handle makepad button widget match
pub fn button(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "text" => prop_text(v),
        "clicked" => action_clicked(v),
        _ => prop_link(prop_name, v)
            .or_else(|_| prop_walk(prop_name, v))
            .or_else(|_| prop_layout(prop_name, v))
            .map_err(Into::into),
    }
}


