use crate::str_to_string_try_from;
use gen_converter::error::Errors;
use gen_parser::Value;
#[allow(unused_imports)]
use std::default;
use std::fmt::Display;

const CHECK: &str = "Check";
const RADIO: &str = "Radio";
const TOGGLE: &str = "Toggle";
const NONE: &str = "None";
/// CheckType is an enum that represents the type of check box.
/// - Check: A check box that can be checked or unchecked.
/// - Radio: A radio button that can be selected or deselected.
/// - Toggle: A toggle button look like a Switch.
/// - None: No check box.
#[derive(Debug, Clone, Default)]
pub enum CheckType {
    #[default]
    Check,
    Radio,
    Toggle,
    None,
}

impl TryFrom<&str> for CheckType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            CHECK => Ok(CheckType::Check),
            RADIO => Ok(CheckType::Radio),
            TOGGLE => Ok(CheckType::Toggle),
            NONE => Ok(CheckType::None),
            _ => {
                return Err(Errors::PropConvertFail(format!(
                    "{} is not a right check type",
                    value
                )))
            }
        }
    }
}

str_to_string_try_from! {CheckType}

impl TryFrom<&Value> for CheckType {
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
                        "{:?} cannot be converted to Makepad::CheckType!",
                        value
                    )))
                })
        }
    }
}

impl Display for CheckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckType::Check => f.write_str(CHECK),
            CheckType::Radio => f.write_str(RADIO),
            CheckType::Toggle => f.write_str(TOGGLE),
            CheckType::None => f.write_str(NONE),
        }
    }
}
