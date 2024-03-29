//! Parse tag
//! - template
//! - script
//! - style
//! ## backup
//! ```code
//! fn start(input: &str) -> IResult<&str, &str> {
//!     delimited(trim(tag(TAG_START)), tag(TEMPLATE), trim(tag(END_SIGN)))(input)
//! }
//! fn end(input: &str) -> IResult<&str, &str> {
//!     delimited(
//!         trim(tag(END_START_SIGN)),
//!         tag(TEMPLATE),
//!         trim(tag(END_SIGN)),
//!     )(input)
//! }
//!
//! fn until_end_template(input: &str) -> IResult<&str, &str> {
//!     let mut rest = input;
//!     let mut remain = "";
//!
//!     loop {
//!         match take_until(END_START_SIGN)(rest) {
//!             Ok((new_rest, taken)) => {
//!                 // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
//!                 match end(new_rest) {
//!                     Ok((final_rest, _)) => {
//!                         //将taken继续放入remain中
//!                         remain = &input[..(remain.len() + taken.len())];
//!                         // 成功找到结束标签，返回累积的内容和剩余的输入
//!                         return Ok((final_rest, remain));
//!                     }
//!                     Err(_) => {
//!                         // 没有找到有效的结束标签，将 "</" 之前的内容加入累积，并继续处理
//!                         remain = &input[..input.len() - new_rest.len() + 2]; // 加 2 是为了包括 "</"
//!                         rest = &new_rest[2..]; // 跳过 "</"，继续尝试
//!                     }
//!                 }
//!             }
//!             Err(e) => return Err(e), // 如果找不到 "</"，则返回错误
//!         }
//!     }
//! }
//!
//! // logic ----------------------------------------------------------------
//! preceded(start, until_end_template)(input)
//! ```

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    multi::many0,
    sequence::delimited,
    IResult,
};

use crate::{ast::Targets, END_SIGN, END_START_SIGN, SCRIPT, STYLE, TAG_START, TEMPLATE};

use super::{comment::parse_offline_comment, trim};

fn delimited_tag<'a>(
    input: &'a str,
    start: &'a str,
    tag_name: &'a str,
) -> IResult<&'a str, &'a str> {
    delimited(trim(tag(start)), tag(tag_name), trim(tag(END_SIGN)))(input)
}

fn start<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    delimited_tag(input, TAG_START, tag_name)
}
pub fn end<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    delimited_tag(input, END_START_SIGN, tag_name)
}

pub fn until_end<'a>(input: &'a str, tag_name: &'a str) -> IResult<&'a str, &'a str> {
    let mut rest = input;
    let mut remain = "";

    loop {
        match take_until(END_START_SIGN)(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match end(new_rest, tag_name) {
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
            Err(e) => return Err(e),
        }
    }
}

/// # parse normal tag
/// example: `<tag_name>xxxx</tag_name>`
/// ## use
/// - parse template tag
/// - parse script tag
/// - parse style tag
pub fn parse_tag<'o>(input: &'o str, tag_name: &'o str) -> IResult<&'o str, &'o str> {
    let (input, _) = start(input, tag_name)?;
    until_end(input, tag_name)
}

fn parse_template_check(input: &str) -> IResult<&str, Targets> {
    match parse_tag(input, TEMPLATE) {
        Ok(_) => Ok(("", Targets::Template(""))),
        Err(e) => Err(e),
    }
}
fn parse_script_check(input: &str) -> IResult<&str, Targets> {
    match parse_tag(input, SCRIPT) {
        Ok(_) => Ok(("", Targets::Script(""))),
        Err(e) => Err(e),
    }
}
fn parse_style_check(input: &str) -> IResult<&str, Targets> {
    match parse_tag(input, STYLE) {
        Ok(_) => Ok(("", Targets::Style(""))),
        Err(e) => Err(e),
    }
}

pub fn parse_tag_check(input: &str) -> IResult<&str, Targets> {
    alt((parse_template_check, parse_script_check, parse_style_check))(input)
}

/// # parse `<template>` tag
/// it can handle many conditions
/// - normal: `<template>xxxx</template>`
/// - strange: `<  template   >xxxx</ template>`
/// ## return
/// `IResult<&str, &str>` parse as => `(_,remain)`
/// > remain: remain &str which has consumed template tag
pub fn parse_template_tag(input: &str) -> IResult<&str, Targets> {
    let (input, remain) = parse_tag(input, TEMPLATE)?;
    Ok((input, Targets::Template(remain)))
}

/// # parse `<script>` tag
pub fn parse_script_tag(input: &str) -> IResult<&str, Targets> {
    let (input, remain) = parse_tag(input, SCRIPT)?;
    Ok((input, Targets::Script(remain)))
}

/// # parse `<style>` tag
pub fn parse_style_tag(input: &str) -> IResult<&str, Targets> {
    let (input, remain) = parse_tag(input, STYLE)?;
    Ok((input, Targets::Style(remain)))
}

/// # parse the whole rsx template file
/// after parse, get `Vec<Targets>`
/// then need to convert `Vec<Targets>` -> `ParseTarget`
pub fn parse_all(input: &str) -> IResult<&str, Vec<Targets>> {
    many0(alt((
        parse_offline_comment,
        parse_template_tag,
        parse_script_tag,
        parse_style_tag,
    )))(input)
}

#[cfg(test)]
mod tag_parser {
    use crate::{
        ast::Targets,
        common::{
            tag::{parse_all, parse_script_tag, parse_tag},
            trim,
        },
        TEMPLATE,
    };

    use super::parse_template_tag;

    #[test]
    fn test_strange_template() {
        let input = r#"<  template >
            <div></div>
        </ template      >"#;
        let (_, inner) = parse_template_tag(input).unwrap();
        assert_eq!(inner, Targets::Template("<div></div>\n        "));
    }

    #[test]
    fn test_template() {
        let input = r#"<template>
            <div></div>
        </template>"#;
        let (_, inner) = parse_template_tag(input).unwrap();
        assert_eq!(inner, Targets::Template("<div></div>\n        "));
    }

    #[test]
    fn test_strange_tag_template() {
        let input = r#"<  template >
            <div></div>
        </ template      >"#;
        let (_, inner) = parse_template_tag(input).unwrap();
        assert_eq!(inner, Targets::Template("<div></div>\n        "));
    }

    #[test]
    fn test_tag_template() {
        let input = r#"<template>
            <div></div>
        </template>"#;
        let (_, inner) = parse_tag(input, TEMPLATE).unwrap();
        assert_eq!(inner, "<div></div>\n        ");
    }

    #[test]
    fn test_tag_script() {
        let input = r#"
        <script>
            const a:&str = "a";
        </script>
        "#;
        let (_, inner) = parse_script_tag(input).unwrap();
        assert_eq!(inner, Targets::Script("const a:&str = \"a\";\n        "));
    }

    #[test]
    fn test_tags() {
        let input = r#"
        <template>
            <window class="ui">
                <view class="body">
                    <button value="Hello world" class="button1" @clicked="handle_actions"/>
                    <text-input value="Click to count" class="input1"/>
                    <label :value="`Counter: ${counter}`" class="label1"/>
                </view>
            </window>
        </template>

        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        <style>
        .ui{
            height : fill;
            width : fill;
            show-bg : true;
            // mix(#7, #3, self.pos.y)
            background-color : linear-gradient(180deg, #7, #3);
            .body{
                flow : down;
                spacing : 20;
                align : center center;
                .button1{ }
                .input1{
                    height : 30;
                    width : 100;
                }
                .label1{
                    color : #ffffff;
                }
            }
        }
        </style>
        "#;
        let (remain, inner) = parse_all(input).unwrap();
        assert_eq!(remain, "");
        dbg!(inner);
    }

    #[test]
    fn test_all() {
        let input = r#"
        //! This is a comment1
        //! This is a comment2
        //! This is a comment3
        <template>
            <window class="ui">
            </window>
        </template>

        // This is line comment
        /// This is a doc comment
        /// hello
        <script>
        let mut counter:usize = 0

        let handle_actions:FnOnce()->() = || {
            counter += 1;
        }
        </script>

        // This is line comment2
        <style>
        .ui{
            height : fill;
            width : fill;
            show-bg : true;
        }
        </style>
        // end of line comment
        "#;
        let (remain, inner) = trim(parse_all)(input).unwrap();
        assert_eq!(remain, "");
        // [parser/src/common/tag.rs:319] inner = [
        //     Comment(
        //         OfflineComment {
        //             value: [
        //                 File(
        //                     "This is a comment1",
        //                 ),
        //                 File(
        //                     "This is a comment2",
        //                 ),
        //                 File(
        //                     "This is a comment3",
        //                 ),
        //             ],
        //             position: AboveTemplate,
        //         },
        //     ),
        //     Template(
        //         "<window class=\"ui\">\n            </window>\n        ",
        //     ),
        //     Comment(
        //         OfflineComment {
        //             value: [
        //                 Normal(
        //                     "This is line comment",
        //                 ),
        //                 Document(
        //                     "This is a doc comment",
        //                 ),
        //                 Document(
        //                     "hello",
        //                 ),
        //             ],
        //             position: AboveScript,
        //         },
        //     ),
        //     Script(
        //         "let mut counter:usize = 0\n\n        let handle_actions:FnOnce()->() = || {\n            counter += 1;\n        }\n        ",
        //     ),
        //     Comment(
        //         OfflineComment {
        //             value: [
        //                 Normal(
        //                     "This is line comment2",
        //                 ),
        //             ],
        //             position: AboveStyle,
        //         },
        //     ),
        //     Style(
        //         ".ui{\n            height : fill;\n            width : fill;\n            show-bg : true;\n        }\n        ",
        //     ),
        //     Comment(
        //         OfflineComment {
        //             value: [
        //                 Normal(
        //                     "end of line comment",
        //                 ),
        //             ],
        //             position: End,
        //         },
        //     ),
        // ]
        dbg!(inner);
    }

    #[test]
    fn test_empty() {
        let input = r#"
        "#;
        let (remain, inner) = trim(parse_all)(input).unwrap();
        assert_eq!(remain, "");
        assert!(inner.is_empty())
    }
}
