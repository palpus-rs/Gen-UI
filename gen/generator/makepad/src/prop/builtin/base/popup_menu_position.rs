use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::str_to_string_try_from;

const ON_SELECTED: &str = "OnSelected";
const BELOW_INPUT: &str = "BelowInput";

#[derive(Debug, Clone, Copy, Default)]
pub enum PopupMenuPosition {
    #[default]
    OnSelected,
    BelowInput,
}

impl TryFrom<&str> for PopupMenuPosition {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ON_SELECTED => Ok(PopupMenuPosition::OnSelected),
            BELOW_INPUT => Ok(PopupMenuPosition::BelowInput),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::PopupMenuPosition!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {PopupMenuPosition}

impl TryFrom<&Value> for PopupMenuPosition {
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
                        "{:?} cannot be converted to Makepad::PopupMenuPosition!",
                        value
                    )))
                })
        }
    }
}

impl Display for PopupMenuPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PopupMenuPosition::OnSelected => f.write_str(ON_SELECTED),
            PopupMenuPosition::BelowInput => f.write_str(BELOW_INPUT),
        }
    }
}
