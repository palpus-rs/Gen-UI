use std::fmt::Display;

use syn::parse::Parse;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DOWN, UP},
};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum EventOrder {
    Down,
    /// default
    Up,
    // List(Vec<>),
}

impl MapValue for EventOrder {
    fn map_value_code(&self) -> String {
        match self {
            EventOrder::Down => "EventOrder::Down".to_string(),
            EventOrder::Up => "EventOrder::Up".to_string(),
        }
    }
}

impl TryFrom<&str> for EventOrder {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            DOWN => Ok(EventOrder::Down),
            UP => Ok(EventOrder::Up),
            _ => Err(Errors::PropConvertFail(format!(
                "{} can not convert to EventOrder",
                value
            ))),
        }
    }
}

str_to_string_try_from! {EventOrder}

impl Display for EventOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventOrder::Down => f.write_str(DOWN),
            EventOrder::Up => f.write_str(UP),
            // EventOrder::List(l) => todo!("EventOrder::List"),
        }
    }
}

impl Parse for EventOrder {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let ident_str = ident.to_string();
        match ident_str.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("{} cannot be converted to EventOrder!", ident_str),
            )),
        }
    }
}
