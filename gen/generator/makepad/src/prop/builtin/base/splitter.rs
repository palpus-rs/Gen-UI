#[allow(unused_imports)]
use std::{default, fmt::Display};

use gen_parser::{Function, Value};
use gen_utils::error::Errors;

use crate::{
    prop::{HORIZONTAL, VERTICAL},
    str_to_string_try_from,
    utils::float_to_str_f64,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum SplitterAxis {
    #[default]
    Horizontal,
    Vertical,
}

impl TryFrom<&str> for SplitterAxis {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            HORIZONTAL => Ok(SplitterAxis::Horizontal),
            VERTICAL => Ok(SplitterAxis::Vertical),
            _ => Err(Errors::PropConvertFail(format!(
                "{} cannot be converted to Makepad::SplitterAxis!",
                value
            ))),
        }
    }
}

str_to_string_try_from! {SplitterAxis}

impl TryFrom<&Value> for SplitterAxis {
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
                        "{:?} cannot be converted to Makepad::SplitterAxis!",
                        value
                    )))
                })
        }
    }
}

impl Display for SplitterAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitterAxis::Horizontal => f.write_str(HORIZONTAL),
            SplitterAxis::Vertical => f.write_str(VERTICAL),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SplitterAlign {
    FromA(f64),
    FromB(f64),
    Weighted(f64),
}

impl TryFrom<&Function> for SplitterAlign {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        let name = value.get_name().to_string();
        let params = value
            .get_params()
            .as_ref()
            .expect("SplitterAlign must have params!")
            .to_vec();
        if params.len() == 1 {
            if name == "from_a" {
                let value = params[0]
                    .parse::<f64>()
                    .expect("SplitterAlign from_a must have a f64 value!");
                return Ok(SplitterAlign::FromA(value));
            } else if name == "from_b" {
                let value = params[0]
                    .parse::<f64>()
                    .expect("SplitterAlign from_a must have a f64 value!");
                return Ok(SplitterAlign::FromB(value));
            } else if name == "weighted" {
                let value = params[0]
                    .parse::<f64>()
                    .expect("SplitterAlign from_a must have a f64 value!");
                if value < 0.0 || value > 1.0 {
                    return Err(Errors::PropConvertFail(format!(
                        "SplitterAlign weighted value must be between 0.0 and 1.0, found {}",
                        value
                    )));
                } else {
                    return Ok(SplitterAlign::Weighted(value));
                }
            }
        }

        return Err(Errors::PropConvertFail(format!(
            "{:?} cannot be converted to Makepad::SplitterAlign!",
            value
        )));
    }
}

impl TryFrom<&Value> for SplitterAlign {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let value = if matches!(value, Value::UnKnown(_)) {
            value.unknown_to_function().unwrap()
        } else {
            value.clone()
        };

        if let Some(s) = value.is_fn_and_get() {
            s.try_into()
        } else {
            return Err(Errors::PropConvertFail(format!(
                "{:?} cannot be converted to Makepad::SplitterAlign!",
                value
            )));
        }
    }
}

impl Display for SplitterAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitterAlign::FromA(a) => f.write_fmt(format_args!("FromA({})", float_to_str_f64(*a))),
            SplitterAlign::FromB(b) => f.write_fmt(format_args!("FromB({})", float_to_str_f64(*b))),
            SplitterAlign::Weighted(w) => {
                f.write_fmt(format_args!("Weighted({})", float_to_str_f64(*w)))
            }
        }
    }
}
