use std::fmt::Display;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{DRAWLIST, NONE, TEXTURE},
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

#[derive(Debug, Clone, PartialEq)]
pub enum ViewOptimize {
    /// defalut
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

impl Display for ViewOptimize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ViewOptimize::None => NONE,
            ViewOptimize::DrawList => DRAWLIST,
            ViewOptimize::Texture => TEXTURE,
        })
    }
}
