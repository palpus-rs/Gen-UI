use super::parse_sign_key;
use crate::FUNCTION_SIGN;
use nom::IResult;

/// ## parse property function key 🆗
/// - `@xxx`
/// - `@xxx_zzz`
pub fn parse_function_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, FUNCTION_SIGN)
}
