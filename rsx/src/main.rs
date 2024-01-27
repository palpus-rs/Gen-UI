use nom::{
    branch::alt, bytes::complete::{tag, take_until, take_while}, character::complete::{alphanumeric1, char, multispace0}, combinator::opt, complete::take, multi::many0, sequence::{delimited, preceded, terminated}, IResult
};

/// match white space like : `\n` | `\t` | `" "` | whitespace
fn parse_whitespace(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn parse_tag_start(input: &str) -> IResult<&str, Tag> {
    let (input,_) = parse_whitespace(input)?;
    let (input,value) = preceded(tag("<"), alphanumeric1)(input)?;
    Ok((input , Tag::Tag(value)))
}

fn parse_attr(input: &str) -> IResult<&str, (&str, &str)> {
    // let (input,_) = parse_whitespace(input)?;
    let (input, key) = alphanumeric1(input)?;
    //匹配 =
    let (input, _) = tag("=")(input)?;
    let (input, value) = delimited(tag("\""), alphanumeric1, take_until("\""))(input)?;
    Ok((input, (key, value)))
}

fn parse_action(input: &str)->IResult<&str,(&str,&str)>{
    let (input, _) = tag("@")(input)?;
    let (input, key) = alphanumeric1(input)?;
    let (input, value) = delimited(tag("\""), alphanumeric1, take_until("\""))(input)?;
    Ok((input, (key, value)))
}

fn parse_attrs(input: &str)->IResult<&str,Tag>{
    let (input,value) = many0(preceded( multispace0, parse_attr))(input)?;
    Ok((input , Tag::Attrs(value)))
}

// fn parse_tag_end(input: &str)->IResult<&str,&str>{
    
// }

enum Tag<'a>{
    Attrs(Vec<(&'a str,&'a str)>),
    Tag(&'a str),
}

fn parse(input: &str)->IResult<&str,Vec<Tag>>{
    // alt((parse_start_tag, parse_self_closing_tag, parse_end_tag))(input)
   many0(alt((parse_tag_start,parse_attrs,parse_action)))(input)
}

fn main() {
    // let input = r#"
    // <window class="ui">
    //     <view class="body">
    //         <button value="Hello world" class="button1" @clicked="handle_actions"/>
    //         <text-input value="Click to count" class="input1"/>
    //         <label :value="`Counter: ${counter}`" class="label1"/>
    //     </view>
    // </window>
    // "#;
    let input = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
    match parse(input) {
        Ok((_s, tags)) => {
            for tag in tags {
                match tag {
                    Tag::Attrs(a) => a.iter().for_each(|x| println!("{:?}",x)),
                    Tag::Tag(t) => println!("{:?}",t),
                };
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
