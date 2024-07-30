//! # Template for ArkUI
//! This fs module is used to parse ArkUI template to AST

use std::collections::HashMap;

use crate::{
    common::{parse_normal, trim},
    ASTNodes, Enum, Function, PropertyKeyType, PropsKey, Struct, Tag, Value,
};
use gen_utils::{common::string::FixedString, error::Error};
use nom::{
    branch::alt,
    bytes::{complete::tag, streaming::take_till},
    character::complete::{alphanumeric1, char},
    combinator::peek,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Err, IResult, Parser,
};

/// ## Tag Name
/// format: `TagName`
#[allow(dead_code)]
fn name(input: &str) -> IResult<&str, &str> {
   parse_key(input)
}
/// ## Tag Args single
/// format: `(xxx)`
#[allow(dead_code)]
fn arg(input: &str) -> IResult<&str, &str> {
    delimited(trim(char('(')), take_till(|c| c == ')'), trim(char(')')))(input)
}
#[allow(dead_code)]
fn chain_fn(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = trim(tag(".")).parse(input)?;
    let (input, key) = name(input)?;
    let (input, value) = arg(input)?;
    Ok((input, (key, value)))
}
/// ## parse this.
/// format: `this.xxx`
fn this(input: &str) -> IResult<&str, &str> {
    let (input, _) = trim(tag("this"))(input)?;
    // preceded(char('.'), alphanumeric1)(input)
    preceded(trim(char('.')), take_till(|c| c == '('))(input)
}

fn parse_snake_case(input: &str) -> IResult<&str, &str> {
    parse_normal(input, '_')
}

fn parse_key(input: &str) -> IResult<&str, &str> {
    trim(alt((parse_snake_case, alphanumeric1)))(input)
}

fn parse_this(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    let (input, (_, value_bind)) = separated_pair(trim(tag("this")), char('.'), parse_key)(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    Ok((input, (Value::bind(value_bind), PropertyKeyType::Bind)))
}

/// is value function
/// format: `()=>this.xxx_fn[(args...)]`
fn parse_fn(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    let input = trim(tag("()"))(input)?.0;
    // remove =>
    let input = trim(tag("=>"))(input)?.0;
    let (input, fn_name) = this(input)?;
    let (input, params) = if input.trim().starts_with('(') {
        let (input, exc_args) = arg(input)?;
        let exc_args = exc_args.split_fixed(",");
        let args = if exc_args.is_empty() {
            None
        } else {
            Some(exc_args)
        };
        (input, args)
    } else {
        (input, None)
    };

    let value_fn = Function {
        name: fn_name.to_string(),
        params,
        is_style: false,
    };

    Ok((input, (value_fn.into(), PropertyKeyType::Function)))
}

pub fn parse_struct(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    let (input, _) = trim(tag("{"))(input)?;

    let (input, fields) = many0(separated_pair(
        parse_key,
        char(':'),
        trim(terminated(
            take_till(|c| c == ',' || c == '}'),
            alt((char(','), char('}'))),
        )),
    ))(input)?;

    let mut is_bind = false;
    let fields = fields
        .into_iter()
        .map(|(key, value_str)| {
            let (value, value_type) = single_value(value_str).unwrap().1;
            if value_type.is_bind() {
                is_bind = true;
            }
            (key, (value, value_type))
        })
        .collect::<Vec<(&str, (Value, PropertyKeyType))>>();

    let value: Struct = fields.into();
    Ok((input, (value.into(), PropertyKeyType::Normal)))
}

/// use serde json to parse and then parse to GenUI Value
pub fn parse_other(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    match serde_json::from_str::<serde_json::Value>(input.trim()) {
        Ok(value) => Ok(("", (value.try_into().unwrap(), PropertyKeyType::Normal))),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/// parse arkui builtin enum
pub fn parse_enum(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    fn follow_part(input: &str) -> IResult<&str, &str> {
        preceded(trim(char('.')), parse_key)(input)
    }

    // split . and then to vec<(&str, &str)>
    let (input, enum_struct) = pair(trim(parse_key), many0(follow_part))(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let enum_struct: Enum = enum_struct.into();

    Ok((input, (enum_struct.into(), PropertyKeyType::Normal)))
}

fn single_value(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
    alt((parse_this, parse_fn, parse_struct, parse_other, parse_enum))(input)
}
#[allow(dead_code)]
fn str_content<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    if terminated(
        delimited(trim(tag("\"")), take_till(|c| c == '\"'), trim(tag("\""))),
        trim(tag(",")),
    )(input)
    .is_ok()
    {
        return terminated(trim(take_till(|c| c == ',')), trim(tag(",")))(input);
    }
    return Err(Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )));
}
#[allow(dead_code)]
fn brace_content<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    // if pair(
    //     delimited(trim(tag("{")), take_till(|c| c == '}'), trim(tag("}"))),
    //     trim(tag(",")),
    // )(input)
    // .is_ok()
    // {
    //     let (input, content) = trim(take_till(|c| c == '}'))(input)?;
    //     return ;
    // }

    fn left(input: &str) -> IResult<&str, &str> {
        trim(tag("{"))(input)
    }
    fn right(input: &str) -> IResult<&str, &str> {
        trim(tag("}"))(input)
    }
    fn till_end(input: &str) -> IResult<&str, &str> {
        delimited(left, take_till(|c| c == '}'), right)(input)
    }

    let (_, content) = till_end(input.trim())?;

    // if input.len() > 1 && input != "," {
    //     // means input still have more words, is nested

    // }

    terminated(tag(format!("{{{}}}", content).as_str()), trim(tag(",")))(input)
}

#[allow(dead_code)]
fn common_content<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    if input.contains('.') {
        return Err(Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    terminated(parse_key, trim(tag(",")))(input)
}

#[allow(dead_code)]
fn enum_content<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    fn follow_part(input: &str) -> IResult<&str, &str> {
        preceded(trim(char('.')), parse_key)(input)
    }
    if !input.contains('.') {
        return Err(Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let (input, content) = take_till(|c| c == ',')(input)?;
    let (res, _) = peek(pair(trim(parse_key), many0(follow_part)))(content)?;
    let (input, _) = trim(tag(","))(input)?;
    Ok((input, res))
}

#[allow(dead_code)]
fn this_content<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let input = input.trim();
    let mut is_this = false;
    if input.starts_with("this") {
        let input2 = input.trim_start_matches("this");
        if input2.trim().starts_with(".") {
            is_this = true;
        }
        if is_this {
            return terminated(tag(input2), trim(tag(",")))(input);
        }
    }
    Err(Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}

/// before use this function, you should format the input: `let mut input = format!("{},", input);`
#[allow(dead_code)]
fn split_args(input: String) -> IResult<String, Vec<String>> {
    // let mut input = format!("{},", input);

    let (input, args) = many0(alt((
        brace_content,
        str_content,
        this_content,
        common_content,
        enum_content,
    )))(&input)
    .unwrap();

    if !input.is_empty() {
        return Err(Err::Error(nom::error::Error::new(
            input.to_string(),
            nom::error::ErrorKind::Tag,
        )));
    }

    Ok((
        input.to_string(),
        args.iter().map(|x| x.to_string()).collect(),
    ))
}

#[allow(dead_code)]
fn to_value<'a>(kv: (&'a str, &'a str)) -> IResult<&'a str, (PropsKey, Value)> {
    // let (input, (mut value, value_type)) =
    //     alt((parse_this, parse_fn, parse_struct, parse_other, parse_enum))(kv.1)?;
    let (input, (mut value, value_type)) = single_value(kv.1)?;

    if !input.is_empty() {
        panic!("still has input can be parse:{}", input);
    }

    if let Value::Struct(struct_value) = &mut value {
        let _ = struct_value.set_name(kv.0);
    }

    Ok((input, (PropsKey::new(kv.0, false, value_type), value)))
}

// fn parse_split(input: &str) -> IResult<&str, (Value, PropertyKeyType)> {
//     dbg!(input);
//     let (input, _) = trim(tag(","))(input)?;

//     Ok((
//         input,
//         (Value::UnKnown(",".to_string()), PropertyKeyType::Normal),
//     ))
// }
/// only use in builtin options parse
#[allow(dead_code)]
fn arg_to_value(input: &str) -> IResult<&str, Vec<(PropsKey, Value)>> {
    // use split_args to split the args
    if !input.is_empty() {
        let input = format!("{},", input);

        let (_, values) = split_args(input).expect("split args error");

        Ok((
            "",
            values
                .into_iter()
                .map(|value| single_value(&value).unwrap().1)
                .map(|(value, value_type)| {
                    (PropsKey::new("options_builtin", false, value_type), value)
                })
                .collect::<Vec<(PropsKey, Value)>>(),
        ))
    } else {
        Ok(("", vec![]))
    }
}


fn parse_tag_start(input: &str) -> IResult<&str, ASTNodes> {
    let (input, tag_name) = name(input)?;
    let (input, arg) = arg(input)?;

    let (_, buitin_options) = arg_to_value(arg)?;

    let tag = Tag::new_tag_props(
        tag_name,
        Some(HashMap::from_iter(buitin_options.into_iter())),
    );
    Ok((input, tag.into()))
}

pub fn parse_tag(input: &str) -> IResult<&str, ASTNodes> {

    fn holder(input: &str) -> IResult<&str, (&str, &str)>{
        pair(trim(tag("{")), trim(tag("}")))(input)
    }

    // parse tag start and get the ast tag
    let (input, mut node) = parse_tag_start(input)?;
    
    let mut input = input.trim();

    if input.starts_with('.') {
        // direct follow `.` means no children should parse fn
        
        // parse property key value ----------------------------------------------
        let (remain, kvs) = many0(chain_fn)(input)?;
        // convert key value to Gen Value ----------------------------------------
        let props = kvs
            .into_iter()
            .map(|kv| to_value(kv).unwrap().1)
            .collect::<HashMap<PropsKey, Value>>();

        node.extend_properties(props);

        input = remain;
    }else if holder(input).is_ok(){
        // means no children
        input = holder(input).unwrap().0;

        if input.starts_with('.') {
            // parse property key value ----------------------------------------------
            let (remain, kvs) = many0(chain_fn)(input)?;
            // convert key value to Gen Value ----------------------------------------
            let props = kvs
                .into_iter()
                .map(|kv| to_value(kv).unwrap().1)
                .collect::<HashMap<PropsKey, Value>>();
    
            node.extend_properties(props);
    
            input = remain;
        }else if input.starts_with('}'){
            // means no children, no props, no same level tag, should return
            return Ok((input, node));
        }else{
            // means no children, no props, but still have same level tag
            dbg!(input);
        }

    }else {
        if input.starts_with("{"){
            input = input.trim_start_matches("{").trim();
            let (input, mut children) = many0(parse_tag)(input)?;
            if !children.is_empty() {
                children
                    .iter_mut()
                    .for_each(|child| child.set_parent(node.clone()));
            }
            node.set_children(children); 
            // if everything is ok, should trim } and return 
            let mut input = trim(tag("}"))(input).unwrap().0;
            if input.is_empty(){
                return Ok((input, node));
            }else{
                // means have props
                if input.starts_with('.') {
                    let (remain, kvs) = many0(chain_fn)(input)?;
                    // convert key value to Gen Value ----------------------------------------
                    let props = kvs
                        .into_iter()
                        .map(|kv| to_value(kv).unwrap().1)
                        .collect::<HashMap<PropsKey, Value>>();
            
                    node.extend_properties(props);
                    input = remain;
                    
                    return Ok((input, node));
                }else{
                    //no props return
                    return Ok((input, node));
                }
            }
        }else{
            return Ok((input, node));
        }
    }

    Ok((input, node))
}

#[allow(dead_code)]
pub fn parse_ark_template(input: &str) -> Result<Vec<ASTNodes>, Error> {
    match many1(parse_tag)(input) {
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
mod test_ark {

    use nom::multi::many0;

    use crate::target::template::ark::split_args;

    use super::{brace_content, enum_content};

    #[test]
    fn test7(){
        let input = r#"
        Root(){
            Window(){
                View(){
                    Label(){}.text("Gen + Makepad + Ark")
                }
            }.id("main_window")
        }.id("ui")
        "#;
        let result = super::parse_ark_template(input);
        // assert!(result.is_ok());
        assert!(result.is_ok());
    }

    #[test]
    fn test6() {
        let input = r#"
            Row(){
                Row(){
                    Text("Hello")
                    Text("Hello2").height("20%")
                }.width("60%")
            }.id("ui")
        "#;
        let result = super::parse_ark_template(input);
        // assert!(result.is_ok());
        assert!(result.is_ok());
    }

    #[test]
    fn test5() {
        let input = r#"
                Column() {
                    Text("Hello world1")
                    Text("Hello world2")
                }.width("80%").height(50)
        "#;
        let result = super::parse_ark_template(input);
        // assert!(result.is_ok());
        assert!(result.is_ok());
    }

    #[test]
    fn test4() {
        let input = r#"
            Row(){
                Column() {}.width("80%").height(50)
                Text("Hello2")
            }
        "#;

        let result = super::parse_ark_template(input);
        assert!(result.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test3() {
        let input = r#"
            Row(){
                Text("Hello1")
                Text("Hello2")
            }
        "#;

        let result = super::parse_ark_template(input);
        assert!(result.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test2() {
        let input = r#"
            Text("Hello")
                .fontSize(20)
                .border({width: 1})
                .textAlign(TextAlign.Center)
        "#;

        let result = super::parse_ark_template(input);
        assert!(result.is_ok());
        
        // assert!(result.is_ok());
    }

    #[test]
    fn test1() {
        let input = r#"
        Row(){
            Text("Hello")
                .fontSize(20)
                .border({width: 1})
                .textAlign(TextAlign.Center)
        }.id("ui")
        "#;

        let result = super::parse_ark_template(input);
        assert!(result.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test0() {
        let input = r#"
        Row()
        "#;
        let input2 = r#"
        Row(){}.width("100%")
        "#;

        let result = super::parse_ark_template(input);
        let result2 = super::parse_ark_template(input2);
        assert!(result.is_ok());
        assert!(result2.is_ok());
        // assert!(result.is_ok());
    }

    #[test]
    fn test_arg_value() {
        // , "Hello"
        let input = r#"TextAlign.Center, 22, "Hello", this.box_height, {width: 10, height: Fill}"#;
        // let input2 = r#""adkjs","#;
        let result = split_args(format!("{},", input));
        // let result = str_content(input2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_enum() {
        let input = r#"TextAlign.Center.Down,"#;
        // let (_, res) = super::parse_enum(input).unwrap();
        let res = enum_content(input);
        assert!(res.is_ok());
        // assert_eq!(res.0.to_string().as_str(), input);
    }

    #[test]
    fn test_struct() {
        let input = r#"{width: 2.0, left: 10, height: this.box_height}, "#;
        let res = brace_content(input);
        assert!(res.is_ok());

        // let (input, res) = super::parse_struct(input).unwrap();
        // dbg!(input, res);
        // dbg!(res.0.to_string());
    }

    #[test]
    fn test_this() {
        let input = r#"box_height"#;
        let res = super::parse_key(input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_value_others() {
        let input1 = r#"20"#;
        let input2 = r#"true"#;
        let input3 = r#"{width: 29}"#;
        let input4 = r#""hello""#;
        let input5 = r#"TextAlign.Center"#;
        let _res1 = super::to_value(("", input1));
        let _res2 = super::to_value(("", input2));
        let _res3 = super::to_value(("border", input3));
        let _res4 = super::to_value(("", input4));
        let _res5 = super::to_value(("", input5));
        // dbg!(res1);
        // dbg!(res2);
        // dbg!(res3);
        // dbg!(res4);
        // dbg!(res5);
    }

    #[test]
    fn test_fn_only() {
        let input = r#"
            () => this.btn_click()"#;
        let res = super::parse_fn(input);
        assert!(res.is_ok());
    }
    #[test]
    fn test_fn_args() {
        let input = r#"
            () => this.btn_click("hello", 15)"#;
        let res = super::parse_fn(input);
        assert!(res.is_ok());
    }
    #[test]
    fn test_fn() {
        let input = r#".onClick(
            () => {
                console.log("Hello")
            }
        )"#;

        let result = super::chain_fn(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_chain() {
        let input = r#".fontSize(20)
        .border({width: 1})"#;

        let result = many0(super::chain_fn)(input);
        assert!(result.is_ok());
    }
}
