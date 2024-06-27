#[allow(unused)]
use std::{default, fmt::Display};

use gen_utils::error::Errors;
use gen_parser::Value;

use crate::{
    prop::{ELLIPSIS, LINE, WORD},
    str_to_string_try_from,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum TextWrap {
    #[default]
    Ellipsis,
    Word,
    Line,
}

impl TryFrom<&str> for TextWrap {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ELLIPSIS => Ok(TextWrap::Ellipsis),
            WORD => Ok(TextWrap::Word),
            LINE => Ok(TextWrap::Line),
            _ => Err(Errors::PropConvertFail(format!(
                "value: {} can not convert to Makepad TextWrap",
                value
            ))),
        }
    }
}

str_to_string_try_from!(TextWrap);

impl TryFrom<&Value> for TextWrap {
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
                        "value: {:?} can not convert to Makepad TextWrap",
                        value
                    )))
                })
        }
    }
}

impl Display for TextWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TextWrap::Ellipsis => ELLIPSIS,
            TextWrap::Word => WORD,
            TextWrap::Line => LINE,
        })
    }
}
