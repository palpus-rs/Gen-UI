use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    IResult,
};

/// parse single hex color
fn hex_digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789abcdefABCDEF"))(input)
}

/// parse 3 hex color 
fn three_hex_digits(input: &str) -> IResult<&str, String> {
    map_res(tuple((hex_digit, hex_digit, hex_digit)), |(a, b, c)| {
        format!("{}{}{}{}{}{}", a, a, b, b, c, c).parse()
    })(input)
}

/// Parse 6 hex color
fn six_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

/// parse hex color
/// - #3       : single
/// - #333     : third
/// - #333333  : sixth
pub fn parse_hex_color(input: &str) -> IResult<&str, String> {
    preceded(
        tag("#"),
        alt((
            map_res(six_hex_digits, |s: &str| s.parse()),
            three_hex_digits,
            map_res(hex_digit, |s|s.parse())
        )),
    )(input)
}
