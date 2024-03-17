use std::{fmt::Display, num::ParseFloatError};

use syn::parse::Parse;

use crate::{error::Errors, str_to_string_try_from};

use super::MapValue;

/// # Makepad Padding
#[derive(Debug, Clone, Copy, Default, PartialEq)]
/// Represents padding values for an widget.
pub struct Padding {
    /// The left padding value.
    pub left: f64,
    /// The top padding value.
    pub top: f64,
    /// The right padding value.
    pub right: f64,
    /// The bottom padding value.
    pub bottom: f64,
}

impl MapValue for Padding {
    fn map_value_code(&self) -> String {
        format!(
            "Padding {{ top: {}, right: {}, bottom: {}, left: {} }}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

impl Padding {
    pub fn new(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Padding {
            left,
            top,
            right,
            bottom,
        }
    }
    pub fn single(space: f64) -> Self {
        Padding::new(space, space, space, space)
    }
    pub fn multi_2(top_bottom: f64, left_right: f64) -> Self {
        Padding::new(left_right, top_bottom, left_right, top_bottom)
    }
    pub fn multi_4(left: f64, top: f64, right: f64, bottom: f64) -> Self {
        Padding::new(left, top, right, bottom)
    }
}

/// Convert padding to Makepad Padding
/// ## single
/// - rsx:      `padding: 10`
/// - makepad:  `padding: 10`
/// ### multi 2
/// - rsx:      `padding: 10 20`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 10, left: 20}`
/// ### multi 4
/// - rsx:      `padding: 10 20 0 29`
/// - makepad:  `padding: {top: 10, right: 20, bottom: 0, left: 29}`
impl TryFrom<&str> for Padding {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try to split ` ` from str
        match value
            .split(' ')
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
        {
            Ok(spaces) => match spaces.len() {
                1 => Ok(Padding::single(spaces[0])),
                2 => Ok(Padding::multi_2(spaces[0], spaces[1])),
                4 => Ok(Padding::multi_4(spaces[0], spaces[1], spaces[2], spaces[3])),
                _ => Err(Errors::PropConvertFail(format!(
                    "{} can not convert to padding",
                    value
                ))),
            },
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to padding",
                value
            ))),
        }
    }
}

str_to_string_try_from! {Padding}

impl Display for Padding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{top: {}, right: {}, bottom: {}, left: {}}}",
            self.top, self.right, self.bottom, self.left
        ))
    }
}

impl Parse for Padding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let value = ident.to_string();
        match value.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("value: {} can not convert to Makepad Padding", value),
            )),
        }
    }
}
