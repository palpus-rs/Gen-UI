#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::str_to_string_try_from;

const WINDOWS_MIN: &str = "WindowsMin";
const WINDOWS_MAX: &str = "WindowsMax";
const WINDOWS_MAX_TOGGLED: &str = "WindowsMaxToggled";
const WINDOWS_CLOSE: &str = "WindowsClose";
const XR_MODE: &str = "XRMode";
const FULLSCREEN: &str = "Fullscreen";

#[derive(Debug, Clone, Copy, Default)]
pub enum DesktopButtonType {
    WindowsMin,
    WindowsMax,
    WindowsMaxToggled,
    WindowsClose,
    XRMode,
    #[default]
    Fullscreen,
}

impl TryFrom<&str> for DesktopButtonType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            WINDOWS_MIN => Ok(DesktopButtonType::WindowsMin),
            WINDOWS_MAX => Ok(DesktopButtonType::WindowsMax),
            WINDOWS_MAX_TOGGLED => Ok(DesktopButtonType::WindowsMaxToggled),
            WINDOWS_CLOSE => Ok(DesktopButtonType::WindowsClose),
            XR_MODE => Ok(DesktopButtonType::XRMode),
            FULLSCREEN => Ok(DesktopButtonType::Fullscreen),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::DesktopButtonType!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {DesktopButtonType}

impl TryFrom<&Value> for DesktopButtonType {
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
                        "{:?} cannot be converted to Makepad::DesktopButtonType!",
                        value
                    )))
                })
        }
    }
}

impl Display for DesktopButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesktopButtonType::WindowsMin => f.write_str(WINDOWS_MIN),
            DesktopButtonType::WindowsMax => f.write_str(WINDOWS_MAX),
            DesktopButtonType::WindowsMaxToggled => f.write_str(WINDOWS_MAX_TOGGLED),
            DesktopButtonType::WindowsClose => f.write_str(WINDOWS_CLOSE),
            DesktopButtonType::XRMode => f.write_str(XR_MODE),
            DesktopButtonType::Fullscreen => f.write_str(FULLSCREEN),
        }
    }
}
