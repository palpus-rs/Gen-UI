mod align;
mod color;
mod flow;
mod margin;
mod padding;
mod size;
mod vecs;

pub use align::{Align, DAlign};
pub use color::Color;
pub use flow::Flow;
pub use margin::Margin;
pub use padding::Padding;
pub use size::Size;
pub use vecs::DVec2;

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
    Flow(Flow),
    DVec2(DVec2),
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
            MakepadPropValue::Flow(flow) => f.write_str(flow.to_string().as_str()),
            MakepadPropValue::DVec2(dv) => f.write_str(dv.to_string().as_str()),
        }
    }
}
