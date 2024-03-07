mod size;
mod margin;

pub use margin::Margin;
pub use size::Size;

use std::fmt::{Display, Write};

use parser::Value;

use crate::error::Errors;

#[derive(Debug, Clone, PartialEq)]
pub enum MakepadPropValue {
    String(String),
    // F64(f64),
    Size(Size),
}

impl Display for MakepadPropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MakepadPropValue::String(s) => f.write_str(s),
            MakepadPropValue::Size(s) => f.write_str(s.to_string().as_str()),
            // MakepadPropValue::F64(num) => f.write_str(num.to_string().as_str()),
            
        }
    }
}
