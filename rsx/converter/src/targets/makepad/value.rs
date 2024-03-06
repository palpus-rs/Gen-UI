use parser::Value;

use crate::error::{self, Errors};

#[derive(Debug, Clone, PartialEq)]
pub enum MakepadPropValue {
    String(String),
    F64(f64),
}
