use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{prop_align, prop_link, prop_padding, prop_text, prop_walk, PropRole},
};

use super::Widgets;

pub fn label(prop_name: &str, v: &Value) -> Result<PropRole, Errors> {
    match prop_name {
        // match to `draw_bg`
        "padding" => prop_padding(v),
        "text" => prop_text(v),
        "align" => prop_align(v),
        // "scroll_bars"=> prop_scroll_bars(v),
        _ => prop_link(prop_name, v)
            .or_else(|_| prop_walk(prop_name, v))
            .or_else(|_| Err(Errors::unmatched_prop(prop_name, Widgets::View))),
    }
}
