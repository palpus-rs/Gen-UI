use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

pub fn prop_flow(value: &Value) -> Result<PropRole, Errors> {
    // match value.is_unknown_and_get() {
    //     Some(s) => match (s).try_into() {
    //         Ok(flow) => Ok(PropRole::normal("flow", MakepadPropValue::Flow(flow))),
    //         Err(e) => Err(e),
    //     },
    //     None => Err(Errors::KnownPropType),
    // }
    let handle = |s: &String| {
        s.try_into()
            .map(|flow| PropRole::normal("flow", MakepadPropValue::Flow(flow)))
            .map_err(Into::into)
    };
    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "flow",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|flow| handle(flow))
            .unwrap_or_else(|| Err(Errors::UnAcceptConvertRange))
    }
}
