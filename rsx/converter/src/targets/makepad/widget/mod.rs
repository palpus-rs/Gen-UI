mod button;
mod common;
mod label;
mod view;
mod window;
mod component;

use std::fmt::Display;

pub use button::button;
pub use label::{generate_label_props, label};
pub use view::view;
pub use window::window;
pub use component::component;

use crate::{str_to_string_try_from, utils::alphabetic::snake_to_camel};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Widgets {
    /// most of the props in the window widget is from view widget
    Window,
    View,
    Button,
    Label,
    Component,
    DefineComponent(String),
}

impl Display for Widgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Widgets::Window => "Window",
            Widgets::View => "View",
            Widgets::Button => "Button",
            Widgets::Label => "Label",
            Widgets::Component => todo!(),
            Widgets::DefineComponent(_) => todo!(),
        })
    }
}

impl From<&str> for Widgets {
    fn from(value: &str) -> Self {
        match snake_to_camel(value).as_str(){
            "Window" => Widgets::Window,
            "View" => Widgets::View,
            "Button" => Widgets::Button,
            "Label" => Widgets::Label,
            "Component" => Widgets::Component,
            _ => Widgets::DefineComponent(value.to_string())
        }
    }
}

impl From<&String> for Widgets {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

// remain props:
// - event_order::list ❓
// - scroll_bars:<ScrollBars> {show_scroll_x: false, show_scroll_y: true}
// - animator
// ```makepad
// animator: {
//     selected = {
//         default: off
//         off = {
//             from: {all: Forward {duration: 0.1}}
//             apply: {draw_check: {selected: 0.0}}
//         }
//         on = {
//             from: {all: Forward {duration: 0.1}}
//             apply: {draw_check: {selected: 1.0}}
//         }
//     }
// }
// ```
