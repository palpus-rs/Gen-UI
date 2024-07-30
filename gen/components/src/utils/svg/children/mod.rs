mod path;
mod circle;
mod rect;
mod line;
mod ellipse;
mod common;


pub use ellipse::*;
pub use line::*;
pub use rect::*;
pub use path::*;
pub use circle::*;
pub use common::*;

#[derive(Debug)]
pub enum Children {
    Circle(Circle),
    Ellipse(Ellipse),
    Line(Line),
    Path(Path),
    Rect(Rect),
    // unsupported now ----------------------
    // Text,
    // TextPath,
    // Tspan,
    // Tref,
    // Polygon,
    // Polyline,
}
