use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::str_to_string_try_from;

const ROUND: &str = "Round";
const TAB: &str = "Tab";
#[derive(Clone, Default, Debug)]
pub struct DrawRadioButton {
    pub radio_type: Option<RadioType>,
}

impl DrawRadioButton {
    pub fn radio_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.radio_type.replace(value.try_into()?);
        Ok(())
    }
}

impl Display for DrawRadioButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(radio_type) = self.radio_type.as_ref() {
            let _ = f.write_fmt(format_args!("radio_type: {},", radio_type));
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone, Default)]
pub enum RadioType {
    #[default]
    Round,
    Tab,
}

impl TryFrom<&str> for RadioType {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ROUND => Ok(RadioType::Round),
            TAB => Ok(RadioType::Tab),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to RadioType!",
                value
            ))),
        }
    }
}

impl Display for RadioType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RadioType::Round => write!(f, "{}", ROUND),
            RadioType::Tab => write!(f, "{}", TAB),
        }
    }
}

str_to_string_try_from! {RadioType}

impl TryFrom<&Value> for RadioType {
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
                        "{:?} cannot be converted to Makepad::RadioType!",
                        value
                    )))
                })
        }
    }
}
