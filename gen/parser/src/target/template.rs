//! 🆗 : 测试完成
//! ⚡️ : faster
use std::collections::HashMap;

use crate::{
    ast::{ASTNodes, PropertyKeyType, PropsKey, Tag},
    common::{
        parse_bind_key, parse_comment as parse_common_comment, parse_function_key, parse_string,
        trim,
    },
    CloseType, Value, END_SIGN, END_START_SIGN, EQUAL_SIGN, SELF_END_SIGN,
};
use gen_utils::error::Error;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{alphanumeric1, char},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use crate::common::parse_normal;

/// ## ⚡️ parse normal label 🆗
/// use in tag_start | tag_end to parse the tag_name
/// ### example
/// - parse xxx
/// - ~parse xxx-zzz~
/// - parse xxx_zzz
#[allow(dead_code)]
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## parse tag start (<) 🆗
/// format : `<tag_name`
/// ### return
/// TemplateASTNode
// #[allow(dead_code)]
// fn parse_tag_start(input: &str) -> IResult<&str, ASTNodes> {
//     let (input, tag_name) = preceded(trim(tag(TAG_START)), parse_tag_name)(input)?;
//     Ok((input, Tag::new_tag_start(tag_name).into()))
//     // Ok((input, TemplateASTNode::new(TemplateNodeType::Tag, tag_name)))
// }

fn parse_tag_start(input: &str) -> IResult<&str, ASTNodes> {
    let (mut remain, (name, props)) =
        preceded(char('<'), tuple((parse_tag_name, parse_properties)))(input)?;
    let props = if props.is_empty() {
        None
    } else {
        Some(
            props
                .into_iter()
                .map(|(key_type, key, value)| (PropsKey::new(key, false, key_type), value))
                .collect::<HashMap<_, _>>(),
        )
    };
    let mut tag = Tag::new_tag_props(name, props);
    // check if remain start with `/>`, if true, is end tag
    if remain.starts_with(SELF_END_SIGN) {
        remain = remain.trim_start_matches(SELF_END_SIGN);
        tag.set_ty(CloseType::SelfClosed);
    } else {
        remain = remain.trim_start_matches(END_SIGN);
    }

    Ok((remain, tag.into()))
}

/// ## parse property key 🆗
/// - normal: k
/// - bind: :k
/// - function: @k
#[allow(dead_code)]
fn parse_property_key(input: &str) -> IResult<&str, (&str, &str)> {
    fn parse_normal_key(input: &str) -> IResult<&str, (&str, &str)> {
        let (input, value) = recognize(pair(
            alphanumeric1,
            take_while_m_n(0, usize::MAX, |c: char| c == '_' || c.is_alphanumeric()),
        ))(input)?;
        Ok((input, ("", value)))
    }
    alt((parse_bind_key, parse_function_key, parse_normal_key))(input)
}

/// ## parse tag property 🆗
/// - normal: `k=\"v\"` value always Value::String
/// - bind: `:k=\"v\"` value flexable (Value::Bind)
/// - function: `@k=\"v\"` value depend on function return (Value:Function)
/// ### return
/// (property_type, property_key, property_value)
#[allow(dead_code)]
fn parse_property(input: &str) -> IResult<&str, (PropertyKeyType, &str, Value)> {
    let (input, (key_type, key)) = parse_property_key(input)?;
    let (input, value) = preceded(tag(EQUAL_SIGN), parse_string)(input)?;
    // parse value
    let key_type: PropertyKeyType = key_type.into();
    let value = key_type.to_value(value);
    Ok((input, (key_type, key, value)))
}

fn parse_properties(input: &str) -> IResult<&str, Vec<(PropertyKeyType, &str, Value)>> {
    many0(trim(parse_property))(input)
}

/// ## parse end tag (`</xxx>`)
// #[allow(dead_code)]
// fn parse_end_tag(input: &str) -> IResult<&str, (&str, &str)> {
//     let (input, value) = trim(delimited(
//         trim(tag(END_START_SIGN)),
//         parse_tag_name,
//         trim(tag(END_SIGN)),
//     ))(input)?;
//     Ok((input, (END_START_SIGN, value)))
// }

/// ## parse tag end 🆗
/// - self end : `/>`
/// - more end : `>` after this , may include children nodes , end is tag end `</xxx>`
#[allow(dead_code)]
fn parse_tag_end(input: &str) -> IResult<&str, &str> {
    alt((tag(SELF_END_SIGN), tag(END_SIGN)))(input)
}

#[allow(dead_code)]
fn parse_comment(input: &str) -> IResult<&str, ASTNodes> {
    match parse_common_comment(input) {
        Ok((input, comment)) => Ok((input, comment.into())),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn to_end_tag(input: &str, tag_name: String) -> IResult<&str, &str> {
    let mut rest = input;
    let mut remain = "";
    let mut nested_count = 0; // 用于计数嵌套标签

    loop {
        match take_until(END_START_SIGN)(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配开始标签，增加嵌套计数

                if taken.trim().starts_with(&(String::from("<") + &tag_name)) {
                    nested_count += 1;
                }
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match delimited(
                    trim(tag(END_START_SIGN)),
                    tag(tag_name.as_str()),
                    trim(tag(END_SIGN)),
                )(new_rest)
                {
                    Ok((final_rest, _)) => {
                        if nested_count == 0 {
                            // 将 taken 继续放入 remain 中
                            remain = &input[..(remain.len() + taken.len())];
                            // 成功找到结束标签，返回累积的内容和剩余的输入
                            return Ok((final_rest, remain));
                        } else {
                            nested_count -= 1; // 减少嵌套计数，继续处理
                            remain = &input[..(remain.len() + taken.len() + tag_name.len() + 3)]; // 加 3 是为了包括 "</"
                            rest = final_rest;
                        }
                        // //将taken继续放入remain中
                        // remain = &input[..(remain.len() + taken.len())];
                        // // 成功找到结束标签，返回累积的内容和剩余的输入
                        // return Ok((final_rest, remain));
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

#[allow(dead_code)]
fn parse_end_tag(input: &str, name: String) -> IResult<&str, (&str, &str)> {
    let (input, value) = trim(delimited(
        trim(tag(END_START_SIGN)),
        tag(&*name),
        trim(tag(END_SIGN)),
    ))(input)?;
    Ok((input, (END_START_SIGN, value)))
}

/// ## parse tag ✅ 🆗
#[allow(dead_code)]
pub fn parse_tag<'a>(
    input: &'a str,
    nests: &mut Vec<String>,
) -> Result<(&'a str, ASTNodes), nom::Err<nom::error::Error<&'a str>>> {
    // parse tag start or comment return ASTNodes, we can use is_tag to check
    let (input, mut ast_node) = trim(alt((parse_comment, parse_tag_start)))(input)?;
    let (is_tag, is_self_closed) = ast_node.is_tag_close();
    if is_tag && !is_self_closed {
        // is tag, nest parse tag
        let tag_name = ast_node.get_tag_name().to_string();
        // trim input and check is start with `</tag_name>`
        let input = match parse_end_tag(input, tag_name.clone()) {
            Ok((input, _)) => {
                return Ok((input, ast_node));
            }
            Err(_) => {
                nests.push(tag_name.clone());
                input
            }
        };

        // has children, parse children
        let (mut input, mut children) = many0(|i| parse_tag(i, nests))(input)?;
        // try clear nests
        if !input.is_empty() {
            nests.iter().for_each(|name| {
                let (remain, _) = parse_end_tag(input, name.clone()).unwrap();
                input = remain;
            });
        }
        if children.is_empty() {
            // no children
            return Ok((input, ast_node));
        }
        if input.is_empty() {
            // set parent
            children
                .iter_mut()
                .for_each(|child| child.set_parent(ast_node.clone()));

            ast_node.set_tag_children(children);

            return Ok((input, ast_node));
        } else {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
    }
    // if is not tag, is comment -> do recursive parse
    Ok((input, ast_node))
}

/// ## parse template Ⓜ️
/// main template parser
#[allow(dead_code)]
pub fn parse_template(input: &str) -> Result<Vec<ASTNodes>, Error> {
    match many1(|s| parse_tag(s, &mut vec![]))(input) {
        Ok((remain, asts)) => {
            if remain.is_empty() {
                return Ok(asts);
            }
            Err(Error::template_parser_remain(remain))
        }
        Result::Err(e) => Err(Error::new(e.to_string().as_str())),
    }
}

#[cfg(test)]
mod template_parsers {

    use std::time::Instant;

    use crate::{ast::PropertyKeyType, target::template::parse_tag_name, Value};

    use super::{
        parse_bind_key, parse_function_key, parse_property, parse_property_key, parse_tag_end,
        parse_tag_start, parse_template,
    };
    #[test]
    fn test_template_nested_same() {
        let template = r#"
        <view id="header">
            <image id="logo"></image>
            <view id="menu_list">
                <label class="menu_item" text="About"></label>
                <label class="menu_item" text="Founders"></label>
                <label class="menu_item" text="Events"></label>
            </view>
        </view>
        "#;

        let _res = parse_template(template);
    }

    #[test]
    fn bad_template3() {
        let template = r#"
        
        "#;
        // dbg!(parse_template(template));
        assert!(parse_template(template).is_err())
    }

    #[test]
    fn bad_template2() {
        let template = r#"
        <input>xxx</input>
        "#;
        // dbg!(parse_template(template));
        assert!(parse_template(template).is_err())
    }

    #[test]
    fn bad_template1() {
        let template = r#"
            </input>
        "#;
        assert!(parse_template(template).is_err());
    }

    #[test]
    fn test_template_all() {
        let template = r#"
        // this is a window
            <window class="ui">
                <view class="body">
                    /// button componet
                    <button value="Hello world" class="button1" @clicked="handle_actions">
                        <div></div>
                        <div />
                    </button>
                    <text_input value="Click to count" class="input1" />
                    <label :value="counter" class="label1" />
                </view>
            </window>
            <text_input value="Click to count" class="input1" />
        "#;
        let t = Instant::now();
        let _ = parse_template(template).unwrap();
        // about 470µs
        dbg!(t.elapsed());
        // let res = res
        //     .into_iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<String>>()
        //     .join("\n");
        // //E:/Rust/try/makepad/rsx/parser/t.rsx
        // let mut f =
        //     File::create("/Users/user/Downloads/beyond-framework-main/rsx/parser/t.html").unwrap();
        // let _ = f.write(res.as_bytes());
        // dbg!(res);
    }
    #[test]
    fn test_parse_template_multi() {
        let tag = r#" 
        //! file!
        <button value="Hello world" class="button1" @clicked="handle_actions"/>"#;

        let res = parse_template(tag).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_template() {
        let tag = r#" <button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let comment = r#"//! file!"#;
        let res1 = parse_template(tag).unwrap();
        let res2 = parse_template(comment).unwrap();
        dbg!(res1);
        dbg!(res2);
    }

    #[test]
    fn test_parse_tag_nesting() {
        let tag1 = r#"
        <view class="body">
            <button value="Hello world" class="button1" @clicked="handle_actions"/>
            <text-input value="Click to count" class="input1"/>
        </view>
        "#;
        assert!(parse_template(tag1).is_ok())
    }

    #[test]
    fn test_parse_tag_normal_close() {
        let tag1 =
            r#"<button :value="hello_world" class="button1" @clicked="handle_actions"></button>"#;
        let res = parse_template(tag1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_tag_close_self() {
        let tag1 = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let res = parse_template(tag1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_tag_end() {
        let self_end = "/>";
        let normal_end = "></normal>";
        let normal_more = "><input /></normal>";
        let res1 = parse_tag_end(self_end).unwrap();
        let res2 = parse_tag_end(normal_end).unwrap();
        let res3 = parse_tag_end(normal_more).unwrap();
        assert_eq!(res1, ("", "/>"));
        assert_eq!(res2, ("</normal>", ">"));
        assert_eq!(res3, ("<input /></normal>", ">"));
    }

    #[test]
    fn test_parse_property() {
        let normal1 = r#"value="hello""#;
        let normal2 = r#"value_key="hello""#;
        let bind1 = r#":value="hello""#;
        let bind2 = r#":value_bind="hello_key""#;
        let function1 = r#"@value="test""#;
        let function2 = r#"@value_func="test_func""#;
        let normal_res1 = parse_property(normal1).unwrap();
        let normal_res2 = parse_property(normal2).unwrap();
        let bind_res1 = parse_property(bind1).unwrap();
        let bind_res2 = parse_property(bind2).unwrap();
        let func_res1 = parse_property(function1).unwrap();
        let func_res2 = parse_property(function2).unwrap();
        assert_eq!(
            normal_res1,
            (
                "",
                (
                    PropertyKeyType::Normal,
                    "value",
                    Value::String("hello".to_string())
                ),
            )
        );
        assert_eq!(
            normal_res2,
            (
                "",
                (
                    PropertyKeyType::Normal,
                    "value_key",
                    Value::String("hello".to_string())
                ),
            )
        );
        assert_eq!(
            bind_res1,
            (
                "",
                (
                    PropertyKeyType::Bind,
                    "value",
                    Value::Bind("hello".to_string())
                ),
            )
        );
        assert_eq!(
            bind_res2,
            (
                "",
                (
                    PropertyKeyType::Bind,
                    "value_bind",
                    Value::Bind("hello_key".to_string())
                ),
            )
        );

        assert_eq!(
            func_res1,
            (
                "",
                (
                    PropertyKeyType::Function,
                    "value",
                    Value::Function("test".into())
                ),
            )
        );
        assert_eq!(
            func_res2,
            (
                "",
                (
                    PropertyKeyType::Function,
                    "value_func",
                    Value::Function("test_func".into())
                ),
            )
        );
    }

    #[test]
    fn test_parse_property_key() {
        let normal1 = r#"value="hello""#;
        let normal2 = r#"value_key="hello""#;
        let bind1 = r#":value="hello""#;
        let bind2 = r#":value_bind="hello_key""#;
        let function1 = r#"@value="test""#;
        let function2 = r#"@value_func="test_func""#;
        let normal_res1 = parse_property_key(normal1).unwrap();
        let normal_res2 = parse_property_key(normal2).unwrap();
        let bind_res1 = parse_property_key(bind1).unwrap();
        let bind_res2 = parse_property_key(bind2).unwrap();
        let func_res1 = parse_property_key(function1).unwrap();
        let func_res2 = parse_property_key(function2).unwrap();
        assert_eq!(normal_res1, ("=\"hello\"", ("", "value",),));
        assert_eq!(normal_res2, ("=\"hello\"", ("", "value_key",),));
        assert_eq!(bind_res1, ("=\"hello\"", (":", "value",),));
        assert_eq!(bind_res2, ("=\"hello_key\"", (":", "value_bind",),));
        assert_eq!(func_res1, ("=\"test\"", ("@", "value",),));
        assert_eq!(func_res2, ("=\"test_func\"", ("@", "value_func",),));
    }

    #[test]
    fn test_parse_function_key() {
        let simple = "@simple";
        let complex = "@complex_test";
        let res1 = parse_function_key(simple).unwrap();
        let res2 = parse_function_key(complex).unwrap();
        assert_eq!(res1, ("", ("@", "simple")));
        assert_eq!(res2, ("", ("@", "complex_test")));
    }

    #[test]
    fn test_parse_bind_key() {
        let simple = ":simple";
        let complex = ":complex_test";
        let res1 = parse_bind_key(simple).unwrap();
        let res2 = parse_bind_key(complex).unwrap();
        assert_eq!(res1, ("", (":", "simple")));
        assert_eq!(res2, ("", (":", "complex_test")));
    }

    #[test]
    fn test_parse_tag_start() {
        let simple = "< button";
        let complex = "< text-input";
        let _res1 = parse_tag_start(simple).unwrap();
        let _res2 = parse_tag_start(complex).unwrap();
        // assert_eq!(
        //     res1,
        //     ("", TemplateASTNode::new(TemplateNodeType::Tag, "button"))
        // );
        // assert_eq!(
        //     res2,
        //     (
        //         "",
        //         TemplateASTNode::new(TemplateNodeType::Tag, "text-input")
        //     )
        // );
    }

    #[test]
    fn test_parse_label() {
        let simple = "button";
        let complex = "text-input";
        let t = Instant::now();
        let res1 = parse_tag_name(simple).unwrap();
        let res2 = parse_tag_name(complex).unwrap();
        let dur = t.elapsed();
        assert_eq!(res1, ("", "button"));
        assert_eq!(res2, ("", "text-input"));
        // 20.129µs | 23.819µs | 16.023µs
        dbg!(dur);
    }
}
