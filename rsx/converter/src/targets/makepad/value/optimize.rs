use std::fmt::Display;

use syn::parse::Parse;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DRAWLIST, NONE, TEXTURE},
};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Optimize {
    View(ViewOptimize),
}

impl MapValue for Optimize {
    fn map_value_code(&self) -> String {
        match self {
            Optimize::View(v) => v.map_value_code(),
        }
    }
}

impl Optimize {
    pub fn view(v: ViewOptimize) -> Self {
        Optimize::View(v)
    }
}

impl Display for Optimize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Optimize::View(v) => f.write_str(v.to_string().as_str()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewOptimize {
    /// defalut
    None,
    DrawList,
    Texture,
}

impl MapValue for ViewOptimize {
    fn map_value_code(&self) -> String {
        match self {
            ViewOptimize::None => "ViewOptimize::None".to_string(),
            ViewOptimize::DrawList => "ViewOptimize::DrawList".to_string(),
            ViewOptimize::Texture => "ViewOptimize::Texture".to_string(),
        }
    }
}

impl TryFrom<&str> for ViewOptimize {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            NONE => Ok(ViewOptimize::None),
            DRAWLIST => Ok(ViewOptimize::DrawList),
            TEXTURE => Ok(ViewOptimize::Texture),
            _ => Err(Errors::PropConvertFail(format!(
                "{} can not convert to ViewOptimize",
                value
            ))),
        }
    }
}

str_to_string_try_from! {ViewOptimize}

impl Display for ViewOptimize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ViewOptimize::None => NONE,
            ViewOptimize::DrawList => DRAWLIST,
            ViewOptimize::Texture => TEXTURE,
        })
    }
}

impl Parse for ViewOptimize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let ident_str = ident.to_string();
        match ident_str.as_str().try_into() {
            Ok(v) => Ok(v),
            Err(_) => Err(syn::Error::new(
                ident.span(),
                format!("{} cannot be converted to ViewOptimize!", ident_str),
            )),
        }
    }
}
