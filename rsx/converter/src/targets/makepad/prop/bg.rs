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
    let handle = |s: &String| {
        s.try_into()
            .map(|draw_bg| PropRole::normal("draw_bg", MakepadPropValue::Color(draw_bg)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "draw_bg",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to draw_bg",
                    value
                )))
            })
    }
}

/// ## Makepad show_bg
/// - true
/// - false
pub fn prop_show_bg(value: &Value) -> Result<PropRole, Errors> {
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<bool>() {
            Ok(b) => Ok(PropRole::normal("show_bg", MakepadPropValue::Bool(b))),
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "show_bg",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_bool_and_get()
            .map(|b| Ok(PropRole::normal("show_bg", MakepadPropValue::Bool(b))))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn prop_color(value: &Value) -> Result<PropRole, Errors> {
    let handle = |s: &String| {
        (s, true)
            .try_into()
            .map(|draw_bg| PropRole::normal("color", MakepadPropValue::Color(draw_bg)))
            .map_err(Into::into)
    };

    if let Some(s) = value.is_unknown_and_get() {
        handle(s)
    } else if let Some(b) = value.is_bind_and_get() {
        Ok(PropRole::bind(
            "color",
            MakepadPropValue::bind_without_value(b),
        ))
    } else {
        value
            .is_string_and_get()
            .map(|s| handle(s))
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to color",
                    value
                )))
            })
    }
}
