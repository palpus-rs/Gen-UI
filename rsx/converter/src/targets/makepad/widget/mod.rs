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
