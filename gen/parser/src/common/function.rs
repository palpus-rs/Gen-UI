use super::parse_sign_key;
use crate::FUNCTION_SIGN;
use nom::{
    bytes::complete::{tag, take_until1, take_while},
    character::complete::char,
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
    // delimited(
    //     tag("("),
    //     delimited(
    //         take_until1("|{"),
    //         take_until1("})"),
    //         take_until1("})"),
    //     ),
    //     take_until1("})"),
    // )(input)

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
        // assert_eq!(output, res);
        dbg!(output);
    }
}
