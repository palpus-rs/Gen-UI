use parser::Value;

use crate::{error::Errors, targets::makepad::value::MakepadPropValue};

use super::PropRole;

/// ## Makepad draw_bg
/// ### convert match
/// #### normal
/// rsx:        `#fff`
///
/// makepad:    `{color:#fff}`
/// #### linear_gradient
/// rsx:        `linear_gradient(deg, color1, color2)`
///
/// makepad:    `fn pixel(self) -> vec4 {
///                 return mix(color1, color2, self.pos.y);
///             }`
pub fn prop_bg(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.try_into() {
            Ok(color) => Ok(PropRole::normal("draw_bg", MakepadPropValue::Color(color))),
            Err(e) => Err(e),
        },
        None => {
            todo!("color bind and function")
            // match value.is_bind_and_get(){
            //     Some(b) => Ok(PropRole::bind("draw_bg", MakepadPropValue::Bind(Box::new(x)))),
            //     None => todo!(),
            // }
        },
    }
}

/// ## Makepad show_bg
/// - true
/// - false
pub fn prop_show_bg(value: &Value) -> Result<PropRole, Errors> {
    match value.is_unknown_and_get() {
        Some(s) => match s.parse::<bool>() {
            Ok(b) => Ok(PropRole::normal("show_bg", MakepadPropValue::Bool(b))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to bool",
                s
            ))),
        },
        None => Err(Errors::KnownPropType),
    }
}
