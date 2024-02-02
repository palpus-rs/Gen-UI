use std::fmt::Display;

use crate::{STYLE_CLASS, STYLE_ID, STYLE_PESUDO};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum StyleType{
     // class: `.`
     Class,
     // id: `#`
     Id,
     // Pseudo: `::`
     Pseudo,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Class
    }
}

impl StyleType {
    pub fn is_class(&self) -> bool {
        matches!(self, Self::Class)
    }
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id)
    }
    pub fn is_pseudo(&self) -> bool {
        matches!(self, Self::Pseudo)
    }
}

impl From<&str> for StyleType {
    fn from(value: &str) -> Self {
        match value {
            STYLE_CLASS => StyleType::Class,
            STYLE_ID => StyleType::Id,
            STYLE_PESUDO => StyleType::Pseudo,
            _ => panic!("Invalid style"),
        }
    }
}

impl Display for StyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            StyleType::Class => STYLE_CLASS,
            StyleType::Id => STYLE_ID,
            StyleType::Pseudo => STYLE_PESUDO,
        };
        f.write_str(res)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Style{
    name:String,
    ty:StyleType
}