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

/// generate makepad label props
pub fn generate_label_props(props: &Vec<PropRole>) -> String {
    // when convert to label prop should focus on draw_text
    let mut text_style = Vec::new();
    let mut draw_text = Vec::new();
    let mut normal = Vec::new();
    for prop in props {
        let (prop_name, _) = prop.is_normal_and_get().unwrap();

        match prop_name {
            "text" => normal.push(prop),
            //--- from DrawText
            "color" | "wrap" => draw_text.push(prop),
            //--- from DrawText::TextStyle
            "font" | "height_factor" | "font_size" | "curve" | "line_spacing" | "top_drop"
            | "brightness" => {
                let _ = text_style.push(prop);
            }
            _ => todo!(),
        };
    }

    let mut draw_text = draw_text
        .into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    if !text_style.is_empty() {
        draw_text.push(format!(
            "text_style: {{ {} }}",
            text_style
                .into_iter()
                .map(|item| item.to_string())
                .collect::<String>()
        ))
    };

    format!(
        "{} draw_text: {{ {} }}",
        normal
            .into_iter()
            .map(|item| item.to_string())
            .collect::<String>(),
        draw_text.join("")
    )
}
