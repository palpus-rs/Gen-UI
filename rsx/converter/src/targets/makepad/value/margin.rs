use std::{fmt::Display, num::ParseFloatError};

use syn::parse::Parse;

use crate::{error::Errors, str_to_string_try_from};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
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

str_to_string_try_from! {Margin}

impl Display for Margin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{top: {}, right: {}, bottom: {}, left: {}}}",
            self.top, self.right, self.bottom, self.left
        ))
    }
}

impl Parse for Margin {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let value = ident.to_string();
        match value.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("value: {} can not convert to Makepad Size", value),
            )),
        }
    }
}
