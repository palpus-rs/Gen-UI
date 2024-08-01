use super::parse_sign_key;
use crate::common::tokenizer::FUNCTION_SIGN;
use nom::{
    bytes::complete::{tag, take_until1},
    sequence::delimited,
    IResult,
};

/// ## parse property function key 🆗
/// - `@xxx`
/// - `@xxx_zzz`
pub fn parse_function_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, FUNCTION_SIGN)
}

pub fn parse_closure_body(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until1("|{")(input)?;

    delimited(tag("|{"), take_until1("})"), tag("})"))(input)
}

#[cfg(test)]
mod test_fn {

    #[test]
    fn closure1() {
        let input = r#"(shader(|self|{
                fn pixel(self) -> vec4{
                    return #FFF;
                }
            })"#;
        let (_, output) = super::parse_closure_body(input).unwrap();
        dbg!(output);
    }
}
