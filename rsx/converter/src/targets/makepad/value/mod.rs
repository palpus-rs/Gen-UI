mod align;
mod color;
mod margin;
mod padding;
mod size;

pub use align::{Align, DAlign};
pub use color::Color;
pub use margin::Margin;
pub use padding::Padding;
pub use size::Size;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum MakepadPropValue {
    String(String),
    F64(f64),
    Size(Size),
    Color(Color),
    Bool(bool),
    Margin(Margin),
    Padding(Padding),
    Align(Align),
}

impl Display for MakepadPropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MakepadPropValue::String(s) => f.write_str(s),
            MakepadPropValue::Size(s) => f.write_str(s.to_string().as_str()),
            MakepadPropValue::Color(c) => {
                f.write_fmt(format_args!("{{ color: {} }}", c.to_string()))
            }
            MakepadPropValue::Bool(b) => f.write_str(&b.to_string()),
            MakepadPropValue::Margin(m) => f.write_str(m.to_string().as_str()),
            MakepadPropValue::Padding(p) => f.write_str(p.to_string().as_str()),
            MakepadPropValue::F64(num) => f.write_str(num.to_string().as_str()),
            MakepadPropValue::Align(a) => f.write_str(a.to_string().as_str()),
        }
    }
}