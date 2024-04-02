use std::{collections::HashMap, default, fmt::Display};

use gen_parser::{PropsKey, Value};
use proc_macro2::TokenTree;

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
    pub fn props(&self, props: HashMap<&PropsKey,&Value>)->Vec<TokenTree> {
       let mut ast = vec![];
        props.iter().for_each(|(prop, value)|{
            let prop_name = prop.name();
            let prop_value = value.is_unknown_and_get().unwrap();
            ast.extend(match self {
                Widget::Window => todo!(),
                Widget::View => view::prop(prop_name, prop_value),
                Widget::Label => todo!(),
                Widget::Button => todo!(),
                Widget::Define(_) => todo!(),
            });
        });
        ast
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
