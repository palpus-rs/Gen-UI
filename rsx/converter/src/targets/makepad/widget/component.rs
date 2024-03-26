//! # Compoent Widget (Only in GenUI)
//! 
//! ## Props (remove id and class)
//! - inherits: Widgets(view, button, label, window...)
//! - props: Struct (the props of the widget, they will be inject into the widget)
//! - actions: Enum (the callbacks of the widget)
//! - 
//! 

use parser::Value;

use crate::{error::Errors, targets::makepad::{action_actions, prop_inherits, prop_props, PropRole}};


pub fn component(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "inherits" => prop_inherits(v),
        "props" => prop_props(v),
        "actions" => action_actions(v),
        _ => Err(Errors::UnMatchedProp(prop_name.to_string(), "component".to_string()))
            
    }
}