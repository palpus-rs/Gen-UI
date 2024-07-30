use nom::{
    bytes::complete::{tag, take_until, take_while_m_n}, character::complete::{alphanumeric1, multispace0}, combinator::recognize, multi::many0, sequence::{delimited, pair, preceded}, IResult
};

/// ## normal parser for easy string and split string
/// depend on what split sign
pub fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alphanumeric1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

pub fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '-')
}

#[allow(unused_mut)]
pub fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}

pub fn parse_normal_key(input: &str) -> IResult<&str, &str> {
    let (input, value) = recognize(pair(
        alphanumeric1,
        take_while_m_n(0, usize::MAX, |c: char| c == '-' || c.is_alphanumeric()),
    ))(input)?;
    Ok((input, value))
}

#[allow(dead_code)]
pub fn parse_property(input: &str) -> IResult<&str, (&str, &str)> {
    let (input,  key) = parse_normal_key(input)?;
    let (input, value) = preceded(tag("="), parse_string)(input)?;
    Ok((input, (key, value)))
}

pub fn parse_properties(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(trim(parse_property))(input)
}

pub fn parse_string(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("\""),
        take_until("\""),
        tag("\""),
    )(input)
}
