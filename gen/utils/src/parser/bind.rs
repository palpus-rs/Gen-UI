use super::parse_sign_key;
use nom::IResult;

use crate::common::tokenizer::BIND_SIGN;

/// ## parse property bind key 🆗
/// - `:xxx`
/// - `:xxx_zzz`
pub fn parse_bind_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, BIND_SIGN)
}
