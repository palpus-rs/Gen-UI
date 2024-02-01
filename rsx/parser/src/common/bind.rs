use super::parse_sign_key;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::recognize,
    sequence::delimited,
    IResult,
};

use crate::BIND_SIGN;

/// ## parse property bind key 🆗
/// - `:xxx`
/// - `:xxx_zzz`
pub fn parse_bind_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, BIND_SIGN)
}
