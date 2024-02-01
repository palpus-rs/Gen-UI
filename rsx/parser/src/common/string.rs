use crate::PROPERTY_VALUE_CONTAIN_SIGN;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::recognize,
    sequence::delimited,
    IResult,
};
/// ## parse string sign single ("\"") 🆗
/// when parse string sign aside
pub fn parse_string_sign_single(input: &str) -> IResult<&str, &str> {
    tag(PROPERTY_VALUE_CONTAIN_SIGN)(input)
}

/// ## parse tag property value 🆗
/// parse : `\"xxx\"` , and return without `\"`
pub fn parse_string(input: &str) -> IResult<&str, &str> {
    // delimited(
    //     parse_property_sign,
    //     recognize(many0(none_of(PROPERTY_VALUE_CONTAIN_SIGN))),
    //     parse_property_sign,
    // )(input)
    delimited(
        parse_string_sign_single,
        take_until(PROPERTY_VALUE_CONTAIN_SIGN),
        parse_string_sign_single,
    )(input)
}

#[cfg(test)]
mod test_string {
    use crate::common::string::{parse_string, parse_string_sign_single};

    #[test]
    fn string() {
        let simple = r#""simple""#;
        let complex = r#""complex_test""#;
        let res1 = parse_string(simple).unwrap();
        let res2 = parse_string(complex).unwrap();
        assert_eq!(res1, ("", "simple"));
        assert_eq!(res2, ("", "complex_test"));
    }

    #[test]
    fn single() {
        let simple = r#""simple""#;
        let complex = r#""complex_test""#;
        let res1 = parse_string_sign_single(simple).unwrap();
        let res2 = parse_string_sign_single(complex).unwrap();
        assert_eq!(res1, ("simple\"", "\"",));
        assert_eq!(res2, ("complex_test\"", "\"",));
    }
}
