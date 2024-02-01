use std::fmt::Display;

use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};

use crate::style::function as parse_function;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: String,
    params: Option<Vec<String>>,
}

impl Function {
    pub fn new(name: &str, params: Vec<String>) -> Self {
        Function {
            name: String::from(name),
            params: Some(params),
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_params(&self) -> &Option<Vec<String>> {
        &self.params
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.get_name();
        let params = match self.get_params() {
            Some(params) => params.join(", "),
            None => "()".to_string(),
        };
        write!(f, "{}({})", name, params)
    }
}

impl From<(&str, Vec<&str>)> for Function {
    fn from(value: (&str, Vec<&str>)) -> Self {
        let params = value
            .1
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        Function::new(value.0, params)
    }
}

impl From<(&str, &str)> for Function {
    fn from(value: (&str, &str)) -> Self {
        // try split &str
        // remove `()`
        let (_, params) = remove_holder(value.1).unwrap();
        let params = params.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
        (value.0, params).into()
    }
}

impl From<&str> for Function {
    fn from(value: &str) -> Self {
        let (input, (_, f)) = parse_function(value).unwrap();
        if input.is_empty() {
            return f.into();
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
