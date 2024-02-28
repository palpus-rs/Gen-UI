//! 🆗 : 测试完成
//! ⚡️ : faster
use std::collections::HashMap;

use crate::{
    common::{parse_bind_key, parse_comment, parse_function_key, parse_string, parse_value, trim},
    Value, END_SIGN, END_START_SIGN, EQUAL_SIGN, SELF_END_SIGN, TAG_START,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{alphanumeric1, multispace0},
    combinator::recognize,
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult,
};

use super::ast::{PropertyKeyType, TemplateASTNode, TemplateNodeType};
use crate::common::parse_normal;

/// ## ⚡️ parse normal label 🆗
/// use in tag_start | tag_end to parse the tag_name
/// ### example
/// - parse xxx
/// - parse xxx-zzz
fn parse_tag_name(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '-')
}

/// ## parse tag start (<) 🆗
/// format : `<tag_name`
/// ### return
/// TemplateASTNode
fn parse_tag_start(input: &str) -> IResult<&str, TemplateASTNode> {
    let (input, value) = preceded(trim(tag(TAG_START)), parse_tag_name)(input)?;
    Ok((input, TemplateASTNode::new(TemplateNodeType::Tag, value)))
}

/// ## parse property key 🆗
/// - normal: k
/// - bind: :k
/// - function: @k
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
fn parse_property(input: &str) -> IResult<&str, (PropertyKeyType, &str, Value)> {
    let (input, (key_type, key)) = parse_property_key(input)?;
    let (input, value) = preceded(tag(EQUAL_SIGN), parse_string)(input)?;
    // parse value
    let key_type: PropertyKeyType = key_type.into();
    let value = key_type.to_value(value);
    Ok((input, (key_type, key, value)))
}

/// ## parse end tag (`</xxx>`)
fn parse_end_tag(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, value) = delimited(tag(END_START_SIGN), parse_tag_name, tag(END_SIGN))(input)?;
    Ok((input, (END_START_SIGN, value)))
}

/// ## parse tag end 🆗
/// - self end : `/>`
/// - more end : `>` after this , may include children nodes , end is tag end `</xxx>`
fn parse_tag_end(input: &str) -> IResult<&str, &str> {
    alt((tag(SELF_END_SIGN), tag(END_SIGN)))(input)
}

/// ## parse tag ✅ 🆗
pub fn parse_tag(input: &str) -> IResult<&str, TemplateASTNode> {
    // get tag beginning
    // let (input, mut tag) = delimited(multispace0, parse_tag_start, multispace0)(input)?;
    let (input, mut ast_tag) = trim(alt((parse_comment, parse_tag_start)))(input)?;
    return if ast_tag.is_tag() {
        // properties
        let mut property_map: HashMap<&str, Value> = HashMap::new();
        let (input, properties) =
            many0(delimited(multispace0, parse_property, multispace0))(input)?;
        for (_key_type, key, value) in properties {
            property_map.insert(key, value);
        }
        ast_tag.properties(property_map);
        // end
        let (input, end) = trim(parse_tag_end)(input)?;
        let (input, children) = match end {
            END_SIGN => {
                let tag_name = format!("</{}>", ast_tag.get_tag_name().unwrap());
                // try find util END tag (</xxx>)
                let (_, middle) = trim(take_until(tag_name.as_str()))(input)?;
                match middle {
                    "" => (middle, None), // no nesting nodes
                    _ => {
                        // has children
                        let (input, mut children) = many0(parse_tag)(middle)?;
                        // for mut child in children {
                        //     child.parent(ast_tag.clone());
                        // }
                        children
                            .iter_mut()
                            .for_each(|child| child.parent(ast_tag.clone()));
                        (input, Some(children))
                    }
                }
            }
            SELF_END_SIGN => (input, None),
            _ => panic!("Invalid end tag"),
        };
        ast_tag.children(children);
        Ok((input, ast_tag))
    } else {
        Ok((input, ast_tag))
    };
}

/// ## parse template Ⓜ️
/// main template parser
pub fn parse_template(input: &str) -> IResult<&str, Vec<TemplateASTNode>> {
    let (input, value) = many0(parse_tag)(input)?;
    Ok((input, value))
}

#[cfg(test)]
mod template_parsers {

    use std::time::Instant;

    use crate::{
        template::{
            ast::{PropertyKeyType, TemplateASTNode, TemplateNodeType},
            parser::parse_tag_name,
        },
        Value,
    };

    use super::{
        parse_bind_key, parse_comment, parse_function_key, parse_property, parse_property_key,
        parse_tag_end, parse_tag_start, parse_template, parse_value,
    };

    #[test]
    fn bad_template2() {
        let template = r#"
        <template>xxx</template>
        "#;
        let (input, ast) = parse_template(template).unwrap();
        dbg!(input);
        dbg!(ast);
    }

    #[test]
    fn bad_template1() {
        let template = r#"
            </template>
        "#;
        let (input, ast) = parse_template(template).unwrap();
        assert!(ast.len() == 0);
    }

    #[test]
    fn test_template_all() {
        let template = r#"
        //! app.rsx
        <template class="app">
            // this is a window
            <window class="ui">
                <view class="body">
                    /// button componet
                    <button value="Hello world" class="button1" @clicked="handle_actions"/>
                    <text-input value="Click to count" class="input1"/>
                    <label :value="counter" class="label1"/>
                </view>
            </window>
        </template>
        "#;
        let t = Instant::now();
        let res = parse_template(template).unwrap();
        dbg!(t.elapsed());
        dbg!(res);
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
        let res = parse_template(tag1).unwrap();
        dbg!(res);
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
    fn test_comment() {
        let normal = "// this is a comment\n";
        let file = "//! this is a file comment\n";
        let doc = "/// this is a doc comment\n";
        let res1 = parse_comment(normal).unwrap();
        let res2 = parse_comment(file).unwrap();
        let res3 = parse_comment(doc).unwrap();
        assert_eq!(
            res1,
            ("\n", TemplateASTNode::comment(" this is a comment", "//"),)
        );
        assert_eq!(
            res2,
            (
                "\n",
                TemplateASTNode::comment(" this is a file comment", "//!"),
            )
        );
        assert_eq!(
            res3,
            (
                "\n",
                TemplateASTNode::comment(" this is a doc comment", "///"),
            )
        );
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
        let res1 = parse_tag_start(simple).unwrap();
        let res2 = parse_tag_start(complex).unwrap();
        assert_eq!(
            res1,
            ("", TemplateASTNode::new(TemplateNodeType::Tag, "button"))
        );
        assert_eq!(
            res2,
            (
                "",
                TemplateASTNode::new(TemplateNodeType::Tag, "text-input")
            )
        );
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
