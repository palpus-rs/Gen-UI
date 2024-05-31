#[allow(unused_imports)]
use std::default;
use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::{
    prop::{BIGGEST, HORIZONTAL, SIZE, SMALLEST, STRETCH, VERTICAL},
    str_to_string_try_from,
};

#[derive(Debug, Clone, Default)]
pub enum ImageFit {
    #[default]
    Stretch,
    Horizontal,
    Vertical,
    Smallest,
    Biggest,
    Size,
}

impl TryFrom<&str> for ImageFit {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            STRETCH => Ok(ImageFit::Stretch),
            HORIZONTAL => Ok(ImageFit::Horizontal),
            VERTICAL => Ok(ImageFit::Vertical),
            SMALLEST => Ok(ImageFit::Smallest),
            BIGGEST => Ok(ImageFit::Biggest),
            SIZE => Ok(ImageFit::Size),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::ImageFit!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {ImageFit}

impl TryFrom<&Value> for ImageFit {
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
                        "{:?} cannot be converted to Makepad::ImageFit!",
                        value
                    )))
                })
        }
    }
}

impl Display for ImageFit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFit::Stretch => f.write_str(STRETCH),
            ImageFit::Horizontal => f.write_str(HORIZONTAL),
            ImageFit::Vertical => f.write_str(VERTICAL),
            ImageFit::Smallest => f.write_str(SMALLEST),
            ImageFit::Biggest => f.write_str(BIGGEST),
            ImageFit::Size => f.write_str(SIZE),
        }
    }
}
