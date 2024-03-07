use std::fmt::Display;

use crate::targets::makepad::constants::{ALL, FILL, FIT};

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Fill,
    Fixed(f64),
    Fit,
    All,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Fill => f.write_str(FILL),
            Size::Fixed(num) => f.write_str(num.to_string().as_str()),
            Size::Fit => f.write_str(FIT),
            Size::All => f.write_str(ALL),
        }
    }
}
