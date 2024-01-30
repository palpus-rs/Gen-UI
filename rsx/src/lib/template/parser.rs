//! 🆗 : 测试完成
//! ⚡️ : faster
use std::collections::HashMap;

use crate::lib::{
    Value, BIND_SIGN, COMMENT_DOCUMENT, COMMENT_FILE, COMMENT_NROMAL, END_SIGN, END_START_SIGN,
    EQUAL_SIGN, FULL_LABEL_ASCII, FUNCTION_SIGN, PROPERTY_VALUE_CONTAIN_SIGN, SELF_END_SIGN,
    TAG_START,
};
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until, take_while, take_while_m_n},
    character::complete::{alphanumeric1, line_ending, multispace0, newline, none_of},
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded},
    IResult,
};

use super::ast::{PropertyKey, PropertyKeyType, TemplateASTNode, TemplateNodeType};

/// ## parse normal label 🆗
/// - parse xxx
/// - parse xxx-zzz
pub fn parse_label_ascii(input: &str) -> IResult<&str, &str> {
    alt((recognize(many1(is_a(FULL_LABEL_ASCII))), alphanumeric1))(input)
}

/// ## ⚡️ parse normal label 🆗
/// (almost) more powerful performance faster than parse_label_ascii average : `24%~32%`
///
/// use in tag_start | property_name | tag_end
///
/// - parse xxx
/// - parse xxx-zzz
pub fn parse_label(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '-')
}

fn parse_normal(input: &str, sign: char) -> IResult<&str, &str> {
    recognize(pair(
        alphanumeric1,
        take_while_m_n(0, usize::MAX, |c: char| c == sign || c.is_alphanumeric()),
    ))(input)
}

/// ## ⚡️ parse normal value 🆗
/// - parse xxx
/// - parse xxx_zzz
pub fn parse_value(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

/// ## parse tag start (<) 🆗
pub fn parse_tag_start(input: &str) -> IResult<&str, TemplateASTNode> {
    let (input, value) = preceded(tag(TAG_START), parse_label)(input)?;
    Ok((input, TemplateASTNode::new(TemplateNodeType::Tag, value)))
}

/// ## parse property sign ("\"") 🆗
fn parse_property_sign(input: &str) -> IResult<&str, &str> {
    tag(PROPERTY_VALUE_CONTAIN_SIGN)(input)
}

/// ## parse tag property value 🆗
/// format: `\"xxx\"`
pub fn parse_property_value(input: &str) -> IResult<&str, &str> {
    delimited(
        parse_property_sign,
        recognize(many0(none_of(PROPERTY_VALUE_CONTAIN_SIGN))),
        parse_property_sign,
    )(input)
}

fn parse_sign_key<'a>(input: &'a str, sign: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    let (input, sign) = tag(sign)(input)?;
    let (input, value) = parse_value(input)?;
    Ok((input, (sign, value)))
}

/// ## parse property bind key 🆗
/// - :xxx
/// - :xxx_zzz
fn parse_bind_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, BIND_SIGN)
}

/// ## parse property function key 🆗
/// - :xxx
/// - :xxx_zzz
fn parse_function_key(input: &str) -> IResult<&str, (&str, &str)> {
    parse_sign_key(input, FUNCTION_SIGN)
}

/// ## parse property key 🆗
/// - normal: k
/// - bind: :k
/// - function: @k
pub fn parse_property_key(input: &str) -> IResult<&str, (&str, &str)> {
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
pub fn parse_property(input: &str) -> IResult<&str, (PropertyKeyType, &str, Value)> {
    let (input, (key_type, key)) = parse_property_key(input)?;
    let (input, value) = preceded(tag(EQUAL_SIGN), parse_property_value)(input)?;
    // parse value
    let key_type: PropertyKeyType = key_type.into();
    let value = key_type.to_value(value);
    Ok((input, (key_type, key, value)))
}

/// ## parse end tag (`</xxx>`)
fn parse_tag_end_tag(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, value) = delimited(tag(END_START_SIGN), parse_label, tag(END_SIGN))(input)?;
    Ok((input, (END_START_SIGN, value)))
}

/// ## parse tag end 🆗
/// - self end : `/>`
/// - more end : `>` after this , may include children nodes , end is tag end `</xxx>`
pub fn parse_tag_end(input: &str) -> IResult<&str, &str> {
    alt((tag(SELF_END_SIGN), tag(END_SIGN)))(input)
}

/// ## parse comments 🆗
/// - //
/// - ///
/// - //!
pub fn parse_comment(input: &str) -> IResult<&str, TemplateASTNode> {
    // let (input,value) = recognize(preceded(
    //     alt((
    //         tag(COMMENT_FILE),
    //         tag(COMMENT_DOCUMENT),
    //         tag(COMMENT_NROMAL),
    //     )),
    //     take_while(|c: char| c != '\n'),
    // ))(input)?;
    let (input, comment_type) = alt((
        tag(COMMENT_FILE),
        tag(COMMENT_DOCUMENT),
        tag(COMMENT_NROMAL),
    ))(input)?;
    let (input, value) = take_while(|c: char| c != '\n')(input)?;

    Ok((input, TemplateASTNode::comment(value, comment_type)))
}

/// ## parse tag ✅ 🆗
pub fn parse_tag(input: &str) -> IResult<&str, TemplateASTNode> {
    // get tag beginning
    let (input, mut tag) = delimited(multispace0, alt((parse_tag_start,parse_comment)), multispace0)(input)?;
    if tag.is_tag(){
        
    }
    // properties
    let mut property_map: HashMap<PropertyKey, Value> = HashMap::new();
    let (input, properties) = many0(delimited(multispace0, parse_property, multispace0))(input)?;
    for (key_type, key, value) in properties {
        property_map.insert(PropertyKey::new(key_type, key), value);
    }
    tag.properties(property_map);
    // end
    let (input, end) = parse_tag_end(input)?;
    let (input ,children) = match end {
        END_SIGN => {
            let tag_name = tag.get_tag_name().unwrap();
            // try find util END tag (</xxx>)
            let (input, middle) = take_until(tag_name)(input)?;
            match middle {
                "" => (input,None), // no nesting nodes
                _ => {
                    let (input, children) = many0(parse_tag)(middle)?;
                    (input,Some(children))
                }
            }
        }
        SELF_END_SIGN => ("",None),
        _ => panic!("Invalid end tag"),
    };
    tag.children(children);
    Ok((input, tag))
}

pub fn parse_template(input: &str) -> IResult<&str, Vec<TemplateASTNode>> {
    let (input, value) = many0(delimited(
        multispace0,
        alt((parse_comment, parse_tag)),
        multispace0,
    ))(input)?;
    Ok((input, value))
}

#[cfg(test)]
mod template_parsers {

    use std::time::Instant;

    use crate::lib::{
        template::ast::{PropertyKeyType, TemplateASTNode, TemplateNodeType},
        Value,
    };

    use super::{
        parse_bind_key, parse_comment, parse_function_key, parse_label, parse_label_ascii,
        parse_property, parse_property_key, parse_property_sign, parse_property_value, parse_tag,
        parse_tag_end, parse_tag_start, parse_template, parse_value,
    };
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
                    <label :value="`Counter: ${counter}`" class="label1"/>
                </view>
            </window>
        </template>
        "#;
        let res = parse_template(template).unwrap();
        dbg!(res);
    }
    #[test]
    fn test_parse_template_multi() {
        let tag = r#" 
        //! file!"
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
        </view>
        "#;
        let res = parse_tag(tag1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_tag_normal_close() {
        let tag1 =
            r#"<button :value="hello_world" class="button1" @clicked="handle_actions"></button>"#;
        let res = parse_tag(tag1).unwrap();
        dbg!(res);
    }

    #[test]
    fn test_parse_tag_close_self() {
        let tag1 = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
        let res = parse_tag(tag1).unwrap();
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
                    Value::Function("test".to_string())
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
                    Value::Function("test_func".to_string())
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
    fn test_parse_property_value() {
        let simple = r#""simple""#;
        let complex = r#""complex_test""#;
        let res1 = parse_property_value(simple).unwrap();
        let res2 = parse_property_value(complex).unwrap();
        assert_eq!(res1, ("", "simple"));
        assert_eq!(res2, ("", "complex_test"));
    }

    #[test]
    fn test_parse_property_sign() {
        let simple = r#""simple""#;
        let complex = r#""complex_test""#;
        let res1 = parse_property_sign(simple).unwrap();
        let res2 = parse_property_sign(complex).unwrap();
        assert_eq!(res1, ("simple\"", "\"",));
        assert_eq!(res2, ("complex_test\"", "\"",));
    }

    #[test]
    fn test_parse_value() {
        let simple = "test";
        let complex = "test_input";
        let more = "test_input_value";
        let res1 = parse_value(simple).unwrap();
        let res2 = parse_value(complex).unwrap();
        let res3 = parse_value(more).unwrap();
        assert_eq!(res1, ("", "test"));
        assert_eq!(res2, ("", "test_input"));
        assert_eq!(res3, ("", "test_input_value"));
    }

    #[test]
    fn test_parse_tag_start() {
        let simple = "<button";
        let complex = "<text-input";
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
        let res1 = parse_label(simple).unwrap();
        let res2 = parse_label(complex).unwrap();
        let dur = t.elapsed();
        assert_eq!(res1, ("", "button"));
        assert_eq!(res2, ("", "text-input"));
        // 20.129µs | 23.819µs ｜ 16.023µs
        dbg!(dur);
    }
    #[test]
    fn test_parse_label_ascii() {
        let simple = "button";
        let complex = "text-input";
        let t = Instant::now();
        let res1 = parse_label_ascii(simple).unwrap();
        let res2 = parse_label_ascii(complex).unwrap();
        let dur = t.elapsed();
        assert_eq!(res1, ("", "button"));
        assert_eq!(res2, ("", "text-input"));
        // 21.198µs | 33.973µs ｜ 76.797µs
        dbg!(dur);
    }
}
