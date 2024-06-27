mod hex;
mod linear;
mod percentage;
mod radial;
mod rgb;
mod rgba;

use std::str::FromStr;

pub use hex::*;
pub use linear::*;
pub use percentage::*;
pub use radial::*;
pub use rgb::*;
pub use rgba::*;

use crate::Function;
use gen_utils::error::Errors;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    IResult,
};

/// ## GenUI 内置颜色类型
/// 颜色写法参考: https://developer.mozilla.org/zh-CN/docs/Web/CSS/color_value
/// - 16进制颜色: #3, #333, #333333
/// - rgb(r, g, b)
/// - rgba(r, g, b, a)
/// - linear_gradient(angle, color percentage, color percentage, ...)
/// - radial_gradient(color percentage, color percentage, ...)
pub enum BuiltinColor {
    /// 16进制颜色
    Hex(Hex),
    /// rgb(r, g, b)
    Rgb(Rgb),
    /// rgba(r, g, b, a)
    Rgba(Rgba),
    /// 线性渐变
    LinearGradient(LinearGradient),
    /// 径向渐变
    RadialGradient(RadialGradient),
}

impl FromStr for BuiltinColor {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 只需要解析16进制颜色
        Hex::from_str(s).map(BuiltinColor::Hex)
    }
}

impl TryFrom<&Function> for BuiltinColor {
    type Error = Errors;

    fn try_from(value: &Function) -> Result<Self, Self::Error> {
        match value.get_name() {
            "rgb" => Rgb::try_from(value).map(BuiltinColor::Rgb),
            "rgba" => Rgba::try_from(value).map(BuiltinColor::Rgba),
            "linear_gradient" => LinearGradient::try_from(value).map(BuiltinColor::LinearGradient),
            "radial_gradient" => RadialGradient::try_from(value).map(BuiltinColor::RadialGradient),
            _ => Err(Errors::ParseError(format!(
                "this function is not a color function: {}",
                value.get_name()
            ))),
        }
    }
}

/// parse single hex color
fn hex_digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789abcdefABCDEF"))(input)
}

/// parse 3 hex color
fn three_hex_digits(input: &str) -> IResult<&str, String> {
    map_res(tuple((hex_digit, hex_digit, hex_digit)), |(a, b, c)| {
        format!("{}{}{}{}{}{}FF", a, a, b, b, c, c).parse()
    })(input)

    // recognize(tuple((
    //     hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    // )))(input)
}

/// Parse 6 hex color
fn six_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

fn eight_hex_digits(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit, hex_digit,
    )))(input)
}

/// parse hex color
/// - #3       : single
/// - #333     : third
/// - #333333  : sixth
/// - #33333333: eighth
pub fn parse_hex_color(input: &str) -> IResult<&str, String> {
    preceded(
        tag("#"),
        alt((
            map_res(eight_hex_digits, |s: &str| s.parse()),
            map_res(six_hex_digits, |s: &str| format!("{}FF", s).parse()),
            three_hex_digits,
            map_res(hex_digit, |s| format!("{}FF", s.repeat(6)).parse()),
        )),
    )(input)
}

/// 将三个参数转换为数字且保证在0-255之间
pub fn trans_color_rgb(v: &str) -> Result<u8, Errors> {
    match v.parse::<u8>() {
        Ok(val) => {
            if val.ge(&0) && val.le(&255) {
                Ok(val)
            } else {
                Err(Errors::ParseError(format!(
                    "parse rgb error: {}, value must between 0-255",
                    val
                )))
            }
        }
        Err(_) => Err(Errors::ParseError(format!(
            "parse rgb error: {}, value must be number",
            v
        ))),
    }
}

pub fn trans_opacity(v: &str) -> Result<f32, Errors> {
    match v.parse::<f32>() {
        Ok(val) => {
            if val.ge(&0.0) && val.le(&1.0) {
                Ok(val)
            } else {
                Err(Errors::ParseError(format!(
                    "parse rgba error: {}, value must between 0-1",
                    val
                )))
            }
        }
        Err(_) => Err(Errors::ParseError(format!(
            "parse rgba error: {}, value must be number",
            v
        ))),
    }
}

#[cfg(test)]
mod test_color {
    use std::str::FromStr;

    use crate::target::parse_style;

    #[test]
    fn test_radial() {
        let style = r#"
        .app{
            background_color : radial_gradient(#7, #3 15%, #f 24%, #d);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            crate::ASTNodes::Tag(_) => todo!(),
            crate::ASTNodes::Comment(_) => todo!(),
            crate::ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.is_fn_and_get().unwrap().clone();
                let color = super::RadialGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_linear2() {
        let style = r#"
        .app{
            background_color : linear_gradient(180deg, #7, #3 15%, #f 24%, #d);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            crate::ASTNodes::Tag(_) => todo!(),
            crate::ASTNodes::Comment(_) => todo!(),
            crate::ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.is_fn_and_get().unwrap().clone();
                let color = super::LinearGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_linear1() {
        let style = r#"
        .app{
            background_color : linear_gradient(180deg, #7 60%, #3 100%);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            crate::ASTNodes::Tag(_) => todo!(),
            crate::ASTNodes::Comment(_) => todo!(),
            crate::ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.is_fn_and_get().unwrap().clone();
                let color = super::LinearGradient::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_rgba() {
        let style = r#"
        .app{
            background_color : rgba(44, 128, 155, 0.5);
        }
        "#;
        let res = parse_style(style).unwrap();

        let node = res[0].clone();

        match node {
            crate::ASTNodes::Tag(_) => todo!(),
            crate::ASTNodes::Comment(_) => todo!(),
            crate::ASTNodes::Style(s) => {
                let style = *s;
                let values = style.get_props().unwrap().values().last().unwrap();
                let fn_v = values.is_fn_and_get().unwrap().clone();
                let color = super::Rgba::try_from(&fn_v).unwrap();
                dbg!(color);
            }
        }
    }

    #[test]
    fn test_percentage() {
        let p = super::Percentage::from_str("11.5%").unwrap();
        let p2 = super::Percentage::from_str("11.5");
        assert_eq!(p.0, 11.5);
        assert!(p2.is_err());
    }
    #[test]
    fn test_hex() {
        let h = super::Hex::from_str("#363").unwrap();
        let h2 = super::Hex::from_str("#3333");
        assert_eq!(h.0, "#336633FF".to_string());
        assert!(h2.is_err());
    }

    #[test]
    fn parse_hex() {
        let h1 = super::parse_hex_color("#3");
        let h2 = super::parse_hex_color("#456");
        let h3 = super::parse_hex_color("#3366aa");
        let h4 = super::parse_hex_color("#23af453a");
        assert_eq!(h1.unwrap().1, "333333FF");
        assert_eq!(h2.unwrap().1, "445566FF");
        assert_eq!(h3.unwrap().1, "3366aaFF");
        assert_eq!(h4.unwrap().1, "23af453a");
    }
}
