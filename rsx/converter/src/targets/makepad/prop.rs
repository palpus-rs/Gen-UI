use std::fmt::Display;

use parser::Value;

use crate::error::Errors;

use super::value::MakepadPropValue;

#[derive(Debug,PartialEq,Clone)]
pub enum PropRole{
    Normal(String,MakepadPropValue),
    Bind(String),
    Function,
    // this means: the current prop is id or class which can link to style properties  (class)
    Context(String),
    // same as Context, but only have one (id)
    Special(String),
}

impl PropRole {
    pub fn is_special(&self) -> bool {
        matches!(self,PropRole::Special(_))
    }
    /// consume self to String
    pub fn to_special(self) -> String{
        match self {
            PropRole::Special(s) => s,
            _ => panic!("Only PropRole::Special can use this function!")
        }
    }
}

impl Display for PropRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropRole::Normal(k, v) => f.write_fmt(format_args!("{}: {},",k,v.to_string())),
            PropRole::Bind(k) => todo!(),
            PropRole::Function => todo!(),
            PropRole::Context(c) => todo!(),
            PropRole::Special(s) => f.write_str(s),
        }
    }
}



pub fn prop_text(key:&str,value:&Value) -> Result<PropRole,Errors>{
    // Unknown -> String
    match value.is_unknown_and_get() {
        Some(s) => Ok(PropRole::Normal(key.to_string(),MakepadPropValue::String(s.to_owned()))),
        None => Err(Errors::KnownPropType),
    }
}

pub fn prop_class(value:&Value) -> Result<PropRole,Errors>{
    match  value.is_unknown_and_get() {
        Some(s) => Ok(PropRole::Context(s.to_owned())),
        None => Err(Errors::KnownPropType),
    }
}

pub fn prop_id(value:&Value) -> Result<PropRole,Errors>{
    match  value.is_unknown_and_get() {
        Some(s) => Ok(PropRole::Special(s.to_owned())),
        None => Err(Errors::KnownPropType),
    }
}