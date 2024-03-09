use std::{fmt::Display, num::ParseFloatError};

use crate::error::Errors;

#[derive(Debug, Clone, Copy, Default,PartialEq)]
pub struct Margin {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Margin {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Margin {
            left,
            top,
            right,
            bottom,
        }
    }
    pub fn single(space: f64) -> Self {
        Margin::new(space, space, space, space)
    }
    pub fn multi_2(top_bottom: f64, left_right: f64) -> Self {
        Margin::new(left_right, top_bottom, left_right, top_bottom)
    }
    pub fn multi_4(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Margin::new(left, top, right, bottom)
    }
}


impl TryFrom<&str> for Margin {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try to split ` ` from str
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(Margin::single(spaces[0])),
                2 => Ok(Margin::multi_2(spaces[0], spaces[1])),
                4 => Ok(Margin::multi_4(spaces[0], spaces[1], spaces[2], spaces[3])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to margin",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to margin",
                value
            ))),
        }
    }
}

impl TryFrom<&String> for Margin {
    type Error = Errors;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}


impl Display for Margin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{top: {}, right: {}, bottom: {}, left: {}}}",
            self.top, self.right, self.bottom, self.left
        ))
    }
}