use gen_converter::error::Errors;
use gen_parser::Value;

pub fn bool_prop<F>(value:&Value, f:F)->Result<(), Errors>
where F: Fn(bool)->() {
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