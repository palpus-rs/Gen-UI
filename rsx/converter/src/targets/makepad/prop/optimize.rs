use parser::Value;

use crate::{
    error::Errors,
    targets::makepad::value::{MakepadPropValue, Optimize},
};

use super::PropRole;

pub fn prop_view_optimize(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(optimize) => Ok(PropRole::normal(
                "optimize",
                MakepadPropValue::Optimize(Optimize::view(optimize)),
            )),
            Err(e) => Err(e),
        },
        None => Err(Errors::KnownPropType),
    }
}
