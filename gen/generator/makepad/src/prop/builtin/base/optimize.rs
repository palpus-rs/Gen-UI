#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_converter::error::Errors;
use gen_parser::Value;
use syn::parse::Parse;

use crate::{
    prop::{DRAWLIST, NONE, TEXTURE},
    str_to_string_try_from,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Optimize {
    View(ViewOptimize),
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ViewOptimize {
    /// defalut
    #[default]
    None,
    DrawList,
    Texture,
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

impl TryFrom<&Value> for ViewOptimize {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(s) = value.is_unknown_and_get() {
            s.try_into()
        } else {
            value
                .is_string_and_get()
                .map(|s| s.try_into())
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{:?} can not convert to ViewOptimize",
                        value
                    )))
                })
        }
    }
}

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
