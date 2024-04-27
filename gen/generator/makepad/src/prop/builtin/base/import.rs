use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;

#[derive(Debug, Default, Clone)]
pub struct LiveDependency(pub String);

impl TryFrom<&str> for LiveDependency {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(LiveDependency(value.to_string()))
    }
}

str_to_string_try_from!(LiveDependency);

impl TryFrom<&Value> for LiveDependency {
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
                        "value: {:?} can not convert to Makepad LiveDependency",
                        value
                    )))
                })
        }
    }
    
}

impl Display for LiveDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dep({})", self.0)
    }
}