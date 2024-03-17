use std::{collections::HashMap, fmt::Display};

use parser::{PropsKey, Value};

use crate::{
    error::Errors,
    targets::makepad::{
        prop_align, prop_draw_text, prop_link, prop_padding, prop_text, prop_walk, PropRole,
    },
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
            .or_else(|_| prop_draw_text(prop_name, v))
            .or_else(|_| Err(Errors::unmatched_prop(prop_name, Widgets::View))),
    }
}

pub fn generate_label_props(props: Vec<PropRole>) -> String {
    // when convert to label prop should focus on draw_text
    let mut text_style = Vec::new();
    for prop in props {
        let (prop_name, prop_value) = prop.is_normal_and_get().unwrap();

        match prop_name {
            "color" => todo!(),
            "wrap" => todo!(),
            //--- from TextStyle
            "font" | "height_factor" | "font_size" | "curve" | "line_spacing" | "top_drop" => {
                let _ = text_style.push(prop);
            }
            _ => todo!(),
        };
    }

    if text_style.is_empty() {
        "".to_string()
    } else {
        format!(
            "text_style: {{ {} }}",
            text_style
                .into_iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
