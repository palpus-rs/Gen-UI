use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{prop_text, PropRole},
};

use super::Widgets;

/// handle makepad button widget match
pub fn button(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        "text" => prop_text(prop_name, v),
        _ => Err(Errors::unmatched_prop(prop_name, Widgets::Button)),
    }
}
