use gen_utils::error::Errors;
use quote::ToTokens;
use std::{fmt::Display, str::FromStr};

use super::{parse_hex_color, Rgb, Rgba};

/// 16进制颜色
#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub String);

impl TryFrom<&str> for Hex {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Hex::from_str(value)
    }
}

impl FromStr for Hex {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((remain, hex)) = parse_hex_color(s) {
            if remain.is_empty() {
                Ok(Hex(format!("#{}", hex)))
            } else {
                Err(Errors::ParseError(format!(
                    "parse hex color error, remain: {}",
                    remain
                )))
            }
        } else {
            Err(Errors::ParseError("parse hex color error".to_string()))
        }
    }
}

impl From<&Rgb> for Hex {
    fn from(value: &Rgb) -> Self {
        Hex(format!("#{:02x}{:02x}{:02x}", value.r, value.g, value.b))
    }
}

impl From<&Rgba> for Hex {
    fn from(value: &Rgba) -> Self {
        Hex(format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            value.r,
            value.g,
            value.b,
            (value.a * 255.0) as u8
        ))
    }
}

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToTokens for Hex {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let s = self.0.as_str();
        tokens.extend(quote::quote! {#s});
    }
}
