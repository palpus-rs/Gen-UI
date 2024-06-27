use std::{fmt::Display, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};
use proc_macro2::TokenStream;

use crate::{common::{parse_value, Special}, target::function};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    params: Option<Vec<String>>,
    /// use to recognize the function is used on the `template` or `style`
    /// if is `style`: `()` should be exist in the function when the function is called (although no args)
    is_style: bool,
}

impl Function {
    pub fn new(name: &str, params: Option<Vec<String>>, is_style: bool) -> Self {
        // check params
        let params = match params {
            Some(p) => {
                if p.is_empty() {
                    None
                } else {
                    Some(p)
                }
            }
            None => None,
        };

        Function {
            name: String::from(name),
            params,
            is_style,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_params(&self) -> &Option<Vec<String>> {
        &self.params
    }
    fn to_params_str(&self) -> Option<String> {
        match self.get_params() {
            Some(params) => Some(params.join(", ")),
            None => None,
        }
    }
    pub fn to_token_easy(&self) -> TokenStream {
        let name = self.get_name();
        let params = self.to_params_str();
        match params {
            Some(p) => format!("{}({})", name, p),
            None => format!("{}()", name),
        }
        .parse()
        .unwrap()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.get_name();
        let params = self.to_params_str();
        if self.is_style {
            match params {
                Some(p) => write!(f, "{}({})", name, p),
                None => f.write_fmt(format_args!("{}()", name)),
            }
        } else {
            // when is template function be called , no args means without `()`
            match params {
                Some(p) => write!(f, "{}({})", name, p),
                None => f.write_str(name),
            }
        }
    }
}

impl From<(&str, Option<Vec<&str>>)> for Function {
    fn from(value: (&str, Option<Vec<&str>>)) -> Self {
        (value.0, value.1, true).into()
    }
}

impl From<(&str, Option<Vec<&str>>, bool)> for Function {
    fn from(value: (&str, Option<Vec<&str>>, bool)) -> Self {
        match value.1 {
            Some(params) => {
                if params.is_empty() {
                    Function::new(value.0, None, value.2)
                } else {
                    let params = params
                        .into_iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                    Function::new(value.0, Some(params), value.2)
                }
            }
            None => Function::new(value.0, None, value.2),
        }
    }
}

impl From<(&str, &str, bool)> for Function {
    fn from(value: (&str, &str, bool)) -> Self {
        // try split &str
        // remove `()`
        if let Ok(_) = Special::from_str(value.0){
            return (value.0, Some(vec![value.1]), value.2).into();
        }
        let (_, params) = remove_holder(value.1).unwrap();
        if params.is_empty() {
            (value.0, None, value.2).into()
        } else {
            let params = params.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            (value.0, Some(params), value.2).into()
        }
    }
}

fn parse_function(input: &str) -> IResult<&str, (&str, &str, bool)> {
    /// not include `()` sign
    fn without_sign(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
        let (input, name) = parse_value(input)?;
        // here input mut be empty
        Ok((input, ("()", (name, "()", Some(false)))))
    }
    match alt((function, without_sign))(input) {
        Ok((input, (_, (name, params, is_style)))) => {
            Ok((input, (name, params, is_style.unwrap())))
        }
        Err(e) => Err(e),
    }
}

impl From<&str> for Function {
    fn from(value: &str) -> Self {
        let (input, f) = parse_function(value).unwrap();
        if input.is_empty() {
            f.into()
        } else {
            panic!("still has input can be parse:{}", input);
        }
    }
}

impl From<&String> for Function {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<String> for Function {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

fn remove_holder(input: &str) -> IResult<&str, &str> {
    delimited(tag("("), take_until(")"), tag(")"))(input)
}

#[cfg(test)]
mod test_func {
    use super::Function;
    #[test]
    fn from_str() {
        let just_name = "hello";
        let f: Function = just_name.into();
        let name = "hello(15)";
        let f2: Function = name.into();
        assert_eq!(f.to_string().as_str(), "hello");
        assert_eq!(f2.to_string().as_str(), "hello(15)");
    }

    #[test]
    fn to_string_easy() {
        let easy = r#"easy()"#;
        let easy_param_single = r#"easy(1)"#;
        let easy_param_multi = r#"easy(1, #fff)"#;
        let easy_s = r#"easy_s(1)"#;
        let easy_with_bind = r#"easy_s(bind1)"#;
        let easy_string_param = r#"easy_s(bind1,"test_hello")"#;
        let func1: Function = easy.into();
        let func2: Function = easy_param_single.into();
        let func3: Function = easy_param_multi.into();
        let func4: Function = easy_s.into();
        let func5: Function = easy_with_bind.into();
        let func6: Function = easy_string_param.into();
        assert_eq!(func1.to_string().as_str(), "easy()");
        assert_eq!(func2.to_string().as_str(), "easy(1)");
        assert_eq!(func3.to_string().as_str(), "easy(1, #fff)");
        assert_eq!(func4.to_string().as_str(), "easy_s(1)");
        assert_eq!(func5.to_string().as_str(), "easy_s(bind1)");
        assert_eq!(func6.to_string().as_str(), "easy_s(bind1, \"test_hello\")");
    }
}
