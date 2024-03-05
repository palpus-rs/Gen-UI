//! 🆗 : 测试完成
//! ⚡️ : faster
use std::collections::HashMap;

use crate::{
    ast::{ASTNodes, PropertyKeyType, PropsKey, Tag},
    common::{
        parse_bind_key, parse_comment as parse_common_comment, parse_function_key, parse_string,
        trim,
    },
    Value, END_SIGN, END_START_SIGN, EQUAL_SIGN, SELF_END_SIGN, TAG_START,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::alphanumeric1,
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};

use crate::common::parse_normal;

/// ## ⚡️ parse normal label 🆗
/// use in tag_start | tag_end to parse the tag_name
/// ### example
/// - parse xxx
/// - parse xxx-zzz
#[allow(dead_code)]
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '-')
}

/// ## parse tag start (<) 🆗
/// format : `<tag_name`
/// ### return
/// TemplateASTNode
#[allow(dead_code)]
fn parse_tag_start(input: &str) -> IResult<&str, ASTNodes> {
    let (input, tag_name) = preceded(trim(tag(TAG_START)), parse_tag_name)(input)?;
    Ok((input, Tag::new_tag_start(tag_name).into()))
    // Ok((input, TemplateASTNode::new(TemplateNodeType::Tag, tag_name)))
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

/// ## parse end tag (`</xxx>`)
#[allow(dead_code)]
fn parse_end_tag(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, value) = delimited(
        trim(tag(END_START_SIGN)),
        parse_tag_name,
        trim(tag(END_SIGN)),
    )(input)?;
    Ok((input, (END_START_SIGN, value)))
}

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

    loop {
        match take_until(END_START_SIGN)(rest) {
            Ok((new_rest, taken)) => {
                // 尝试匹配结束标签，如果失败，说明 "</" 不是有效的结束标签的开始
                match delimited(
                    trim(tag(END_START_SIGN)),
                    tag(tag_name.as_str()),
                    trim(tag(END_SIGN)),
                )(new_rest)
                {
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

/// ## parse tag ✅ 🆗
#[allow(dead_code)]
pub fn parse_tag(input: &str) -> IResult<&str, ASTNodes> {
    // get tag beginning
    // let (input, mut tag) = delimited(multispace0, parse_tag_start, multispace0)(input)?;
    let (input, mut ast_tag) = trim(alt((parse_comment, parse_tag_start)))(input)?;
    return if ast_tag.is_tag() {
        // properties
        let (input, properties) =
            // many0(delimited(multispace0, parse_property, multispace0))(input)?;
            many0(trim(parse_property))(input)?;
        let tag_properties = if properties.is_empty() {
            None
        } else {
            let mut property_map = HashMap::new();
            for (key_type, key, value) in properties {
                property_map.insert(PropsKey::new(key, false, key_type), value);
            }
            Some(property_map)
        };
        ast_tag.set_tag_properties(tag_properties);
        // end
        let (input, end) = trim(parse_tag_end)(input)?;
        ast_tag.set_tag_type(end.into());
        let (input, children) = match end {
            END_SIGN => {
                let (remain, middle) = to_end_tag(input, ast_tag.get_tag_name().to_string())?;
                match middle {
                    "" => {
                        if remain.is_empty() {
                            (middle, None)
                        } else {
                            (remain, None)
                        }
                    } // no nesting nodes
                    _ => {
                        // has children
                        let (input, mut children) = many0(parse_tag)(middle)?;
                        if children.is_empty() {
                            (input, None)
                        } else {
                            children
                                .iter_mut()
                                .for_each(|child| child.set_parent(ast_tag.clone()));
                            (remain, Some(children))
                        }
                    }
                }
            }
            SELF_END_SIGN => (input, None),
            _ => panic!("Invalid end tag"),
        };
        if children.is_some() {
            let _ = ast_tag.set_tag_children(
                children
                    .unwrap()
                    .into_iter()
                    .map(|item| item.into())
                    .collect::<Vec<ASTNodes>>(),
            );
        };
        Ok((input, ast_tag))
    } else {
        Ok((input, ast_tag))
    };
}

/// ## parse template Ⓜ️
/// main template parser
#[allow(dead_code)]
pub fn parse_template(input: &str) -> Result<Vec<ASTNodes>, crate::error::Error> {
    match many1(parse_tag)(input) {
        Ok((remain, asts)) => {
            if remain.is_empty() {
                return Ok(asts);
            }
            Err(crate::error::Error::template_parser_remain(remain))
        }
        Result::Err(_) => Err(crate::error::Error::new("error parsing template")),
    }
}

#[cfg(test)]
mod template_parsers {

    use std::{fs::File, io::Write, time::Instant};

    use crate::{ast::PropertyKeyType, target::template::parse_tag_name, Value};

    use super::{
        parse_bind_key, parse_function_key, parse_property, parse_property_key, parse_tag_end,
        parse_tag_start, parse_template,
    };

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
                    <text-input value="Click to count" class="input1" />
                    <label :value="counter" class="label1" />
                </view>
            </window>
            <text-input value="Click to count" class="input1" />
        "#;
        let t = Instant::now();
        let res = parse_template(template).unwrap();
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
