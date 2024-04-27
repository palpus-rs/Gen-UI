use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;

use super::LiveDependency;

#[derive(Clone,Debug)]
pub struct Font {
    pub path: LiveDependency
}

impl TryFrom<&str> for Font {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Font {
            path: LiveDependency::try_from(value)?
        })
    }
    
}

str_to_string_try_from!(Font);

impl TryFrom<&Value> for Font {
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
                        "value: {:?} can not convert to Makepad Font",
                        value
                    )))
                })
        }
    }
    
}

impl  Display for Font{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.fmt(f)
    }
}