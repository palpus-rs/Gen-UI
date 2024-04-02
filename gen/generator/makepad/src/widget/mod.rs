use std::default;

mod button;
mod label;
mod view;
mod window;
mod define;

// pub use define::*;
// pub use button::*;
// pub use label::*;
// pub use view::*;
// pub use window::*;


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
    pub fn ast(&self){
        match self {
            Widget::Define(name) => define::ast(name),
            _=>todo!()
        }
    }
}