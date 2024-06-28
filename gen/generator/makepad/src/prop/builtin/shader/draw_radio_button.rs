use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use crate::{str_to_string_try_from, widget::utils::f32_prop};

use super::draw_quad::DrawQuad;

const ROUND: &str = "Round";
const TAB: &str = "Tab";
#[derive(Clone, Default, Debug)]
pub struct DrawRadioButton {
    pub draw_super: DrawQuad,
    pub radio_type: Option<RadioType>,
    pub hover: Option<f32>,
    pub focus: Option<f32>,
    pub selected: Option<f32>,
}

impl DrawRadioButton {
    pub fn radio_type(&mut self, value: &Value) -> Result<(), Errors> {
        self.radio_type.replace(value.try_into()?);
        Ok(())
    }
    pub fn hover(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.hover.replace(f);
        })
    }
    pub fn focus(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.focus.replace(f);
        })
    }
    pub fn selected(&mut self, value: &Value) -> Result<(), Errors> {
        f32_prop(value, |f| {
            self.selected.replace(f);
        })
    }
    pub fn color(&mut self, value: &Value) -> Result<(), Errors> {
        let quad = DrawQuad::try_from(value)?;
        self.draw_super = quad;
        Ok(())
    }
}

impl Display for DrawRadioButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(self.draw_super.to_string().as_str());
        if let Some(radio_type) = self.radio_type.as_ref() {
            let _ = f.write_fmt(format_args!("radio_type: {},", radio_type));
        }
        if let Some(hover) = &self.hover {
            let _ = f.write_fmt(format_args!("hover: {},", hover));
        }
        if let Some(focus) = &self.focus {
            let _ = f.write_fmt(format_args!("focus: {},", focus));
        }
        if let Some(selected) = &self.selected {
            let _ = f.write_fmt(format_args!("selected: {},", selected));
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
