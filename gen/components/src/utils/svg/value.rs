use std::{
    collections::HashMap, fmt::{Debug, Display}, str::FromStr
};

use crate::impl_auto_value;

pub trait AutoValue: PartialEq + ToString + FromStr + Debug {
    fn as_auto(v: Self) -> Auto<Self>;
}

/// Value types for SVG values that can be auto or other types.
#[derive(Debug, Clone)]
pub struct Auto<T: AutoValue>(pub Option<T>);

impl<T: AutoValue> Auto<T>{
    pub fn get_from_map(map: &HashMap<&str, &str>, k: &str) -> Auto<T>{
        if let Some(v) = map.get(k){
            Auto::from_str(v).unwrap()
        }else{
            Auto(None)
        }
    }
}

impl<T: AutoValue> PartialEq for Auto<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            (None, Some(v)) | (Some(v), None) => v.to_string().eq("auto"),
        }
    }
}

impl<T: AutoValue> Default for Auto<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: AutoValue> std::str::FromStr for Auto<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq("auto") {
            Ok(Auto(None))
        } else {
            match s.parse::<T>() {
                Ok(f) => Ok(Auto(Some(f))),
                Err(_) => Err(format!("parse error{:?}", s)),
            }
        }
    }
}

impl<T: AutoValue> Display for Auto<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_none() {
            write!(f, "auto")
        } else {
            write!(f, "{}", self.0.as_ref().unwrap().to_string())
        }
    }
}

impl_auto_value!(f32, i32, String, bool, u32);
