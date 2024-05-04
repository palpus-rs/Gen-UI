use gen_converter::error::Errors;
use gen_parser::Value;
use proc_macro2::TokenStream;

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

pub fn bind_prop_value(value: &Value, is_prop: bool, ident: &str) -> String {
    return if let Some(v) = value.is_bind_and_get() {
        if is_prop && ident.starts_with(ident) {
            // 说明这个绑定的属性是从外部传入的，需要将定义的首个prefix转为self
            v.replacen(ident, "self", 1)
        } else {
            v.to_string()
        }
    } else {
        panic!("prop value is not bind")
    };
}

pub fn quote_prop(keys: Vec<&str>, value: &str) -> TokenStream {
    let mut result = String::new();

    // 迭代keys，逐一构建字符串
    for &key in keys.iter() {
        if !result.is_empty() {
            result.push_str(": {");
        }
        result.push_str(key);
    }

    // 添加最内层的值
    result.push_str(" : (");
    result.push_str(value);
    result.push_str("),");

    // 根据keys的数量，添加相应数量的闭括号
    for _ in 0..keys.len() - 1 {
        result.push_str("},");
    }

    result.parse().unwrap()
}


#[cfg(test)]
mod test_utils{
    #[test]
    fn test_quote_prop(){
        let keys = vec!["a", "b", "c"];
        let value = "1";
        let result = super::quote_prop(keys, value);
        assert_eq!(result.to_string(), "a : { b : { c : (1) , } , } ,");
    }
}