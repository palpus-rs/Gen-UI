mod button;
mod window;
mod view;
mod label;

use std::fmt::Display;

pub use button::button;
pub use window::window;

#[derive(Debug,Clone,PartialEq)]
pub enum Widgets{
    Window,
    Button
}

impl Display for Widgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Widgets::Window =>"Window",
            Widgets::Button => "Button"
        })
    }
}