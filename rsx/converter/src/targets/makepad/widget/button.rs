use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{
        action::action_clicked, prop_layout, prop_link, prop_text, prop_walk, PropRole,
    },
};

use super::Widgets;

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
