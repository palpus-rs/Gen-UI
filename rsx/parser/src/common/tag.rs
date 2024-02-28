use nom::{
    bytes::complete::{tag, take_till, take_till1, take_until, take_until1, take_while},
    character::complete::anychar,
    combinator::{cut, map_res, peek},
    error::context,
    multi::{many0, many_till},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

use crate::{END_SIGN, END_START_SIGN, TAG_START, TEMPLATE};

use super::trim;

fn until_end<'a,P,O>(input: &'a str , mut end:P) -> IResult<&'a str, &'a str> 
where P:FnMut(&'a str) -> IResult<&'a str, O>{
    let mut rest = input;
    let mut remain = "";

    loop {
        match take_until("</")(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match end(new_rest) {
                    Ok((final_rest, _)) => {
                        //将taken继续放入remain中
                        remain = &input[..(remain.len() + taken.len())];
                        // 成功找到结束标签，返回累积的内容和剩余的输入
                        return Ok((final_rest, remain));
                    }
                    Err(_) => {
                        // 没有找到有效的结束标签，将 "</" 之前的内容加入累积，并继续处理
                        remain = &input[..input.len() - new_rest.len() + 2]; // 加 2 是为了包括 "</"
                        rest = &new_rest[2..]; // 跳过 "</"，继续尝试
                    }
                }
            }
            Err(e) => return Err(e), // 如果找不到 "</"，则返回错误
        }
    }
}


/// parse `<template>` tag
/// ## return 
/// IResult<&str, &str> parse as => `(_,remain)`
fn parse_template_tag(input: &str) -> IResult<&str, &str> {
    fn start(input: &str) -> IResult<&str, &str> {
        delimited(trim(tag(TAG_START)), tag(TEMPLATE), trim(tag(END_SIGN)))(input)
    }
    fn end(input: &str) -> IResult<&str, &str> {
        delimited(
            trim(tag(END_START_SIGN)),
            tag(TEMPLATE),
            trim(tag(END_SIGN)),
        )(input)
    }

    fn until_end_template(input: &str) -> IResult<&str, &str> {
        let mut rest = input;
        let mut remain = "";

        loop {
            match take_until("</")(rest) {
                Ok((new_rest, taken)) => {
                    // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                    match end(new_rest) {
                        Ok((final_rest, _)) => {
                            //将taken继续放入remain中
                            remain = &input[..(remain.len() + taken.len())];
                            // 成功找到结束标签，返回累积的内容和剩余的输入
                            return Ok((final_rest, remain));
                        }
                        Err(_) => {
                            // 没有找到有效的结束标签，将 "</" 之前的内容加入累积，并继续处理
                            remain = &input[..input.len() - new_rest.len() + 2]; // 加 2 是为了包括 "</"
                            rest = &new_rest[2..]; // 跳过 "</"，继续尝试
                        }
                    }
                }
                Err(e) => return Err(e), // 如果找不到 "</"，则返回错误
            }
        }
    }

    // logic ----------------------------------------------------------------
    preceded(start, until_end_template)(input)
}


#[cfg(test)]
mod tag_parser {
    use super::{parse_template_tag};

    #[test]
    fn test_template() {
        let input = r#"<template>
            <div></div>
        </template>"#;
        let (_,inner) = parse_template_tag(input).unwrap();
        assert_eq!(inner, "<div></div>\n        ");
    }
}
