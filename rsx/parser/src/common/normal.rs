use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{alphanumeric1, multispace0},
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

/// ## normal parser for easy string and split string
/// depend on what split sign
pub fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alphanumeric1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

/// ## ⚡️ parse normal value 🆗
/// use in property | value
/// - parse xxx
/// - parse xxx_zzz
pub fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## parse sign then get parse_value
/// format: `_xxx_zzz` | `@sss_vvv`
pub fn parse_sign_key<'a>(input: &'a str, sign: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let (input, sign) = tag(sign)(input)?;
    let (input, value) = parse_value(input)?;
    Ok((input, (sign, value)))
}

/// ## trim any parser left and right multispace(if exist)
pub fn trim<'a, P, O>(mut parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, parser, multispace0)
}

#[cfg(test)]
mod normal {}
