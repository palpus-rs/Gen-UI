use std::{default, fmt::Display};

mod button;
mod define;
mod label;
mod view;
mod window;

// pub use define::*;
// pub use button::*;
// pub use label::*;
// pub use view::*;
// pub use window::*;

const WINDOW: &str = "Window";
const VIEW: &str = "View";
const LABEL: &str = "Label";
const BUTTON: &str = "Button";

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Widget {
    Window,
    #[default]
    View,
    Label,
    Button,
    Define(String),
}

impl Widget {
    pub fn ast(&self) {
        match self {
            Widget::Define(name) => todo!(),
            _ => todo!(),
        }
    }
    pub fn prop(&self) {
        match self {
            Widget::Window => window::prop(),
            Widget::View => todo!(),
            Widget::Label => todo!(),
            Widget::Button => todo!(),
            Widget::Define(_) => todo!(),
        }
    }
}

impl From<&str> for Widget {
    fn from(value: &str) -> Self {
        match value {
            WINDOW => Widget::Window,
            VIEW => Widget::View,
            LABEL => Widget::Label,
            BUTTON => Widget::Button,
            _ => Widget::Define(value.to_string()),
        }
    }
}

impl Display for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Widget::Window => WINDOW,
            Widget::View => VIEW,
            Widget::Label => LABEL,
            Widget::Button => BUTTON,
            Widget::Define(d) => d,
        })
    }
}
