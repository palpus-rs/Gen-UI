mod button;
mod common;
mod label;
mod view;
mod window;

use std::fmt::Display;

pub use button::button;
pub use view::view;
pub use window::window;

#[derive(Debug, Clone, PartialEq)]
pub enum Widgets {
    /// most of the props in the window widget is from view widget
    Window,
    View,
    Button,
}

impl Display for Widgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Widgets::Window => "Window",
            Widgets::View => "View",
            Widgets::Button => "Button",
        })
    }
}

// remain props:
// - Layout::scroll
