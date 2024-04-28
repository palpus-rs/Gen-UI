use gen_converter::error::Errors;
use gen_parser::Value;

pub fn bool_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(bool) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<bool>() {
            Ok(b) => {
                f(b);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else {
        value
            .is_bool_and_get()
            .map(|b| {
                f(b);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn f64_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f64) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f64>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else if let Some(d) = value.is_double_and_get() {
        f(d);
        Ok(())
    } else {
        value
            .is_float_and_get()
            .map(|b| {
                f(b as f64);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn f32_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f32) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f32>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else {
        value
            .is_float_and_get()
            .map(|b| {
                f(b);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn string_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&str) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        f(s);
        Ok(())
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                f(s);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}
