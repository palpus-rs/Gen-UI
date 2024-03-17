use std::fmt::Display;

use crate::{
    error::Errors,
    str_to_string_try_from,
    targets::makepad::constants::{ELLIPSIS, LINE, WORD},
};

use super::MapValue;

#[derive(Debug, Clone, PartialEq)]
pub enum TextWrap {
    Ellipsis,
    Word,
    Line,
}

impl MapValue for TextWrap {
    fn map_value_code(&self) -> String {
        match self {
            TextWrap::Ellipsis => "TextWrap::Ellipsis".to_string(),
            TextWrap::Word => "TextWrap::Word".to_string(),
            TextWrap::Line => "TextWrap::Line".to_string(),
        }
    }
}

impl TryFrom<&str> for TextWrap {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ELLIPSIS => Ok(TextWrap::Ellipsis),
            WORD => Ok(TextWrap::Word),
            LINE => Ok(TextWrap::Line),
            _ => Err(Errors::PropConvertFail(format!(
                "{} can not convert to TextWrap",
                value
            ))),
        }
    }
}

str_to_string_try_from! {TextWrap}

impl Display for TextWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TextWrap::Ellipsis => ELLIPSIS,
            TextWrap::Word => WORD,
            TextWrap::Line => LINE,
        })
    }
}
