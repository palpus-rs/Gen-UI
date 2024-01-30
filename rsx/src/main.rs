// use std::fs::read_to_string;

// use nom::{
//     branch::alt,
//     bytes::complete::{escaped_transform, is_a, is_not, tag, take_until, take_while},
//     character::complete::{alphanumeric0, alphanumeric1, anychar, char, multispace0, none_of},
//     combinator::{self, opt, recognize, value, verify},
//     complete::take,
//     multi::{many0, many1},
//     sequence::{delimited, preceded, terminated},
//     IResult,
// };

// mod lib;

// /// match white space like : `\n` | `\t` | `" "` | whitespace
// fn parse_whitespace(input: &str) -> IResult<&str, &str> {
//     multispace0(input)
// }

// fn parse_tag_start(input: &str) -> IResult<&str, Tag> {
//     // let (input, _) = parse_whitespace(input)?;
//     let (input, value) = preceded(
//         tag("<"),
//         alt((
//             recognize(many1(is_a(
//                 "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-",
//             ))),
//             alphanumeric1,
//         )),
//     )(input)?;
//     Ok((input, Tag::Tag(value)))
// }

// fn parse_attr(input: &str) -> IResult<&str, Tag> {
//     let (input, flag) = opt(is_a("@:"))(input)?;

//     let (input, key) = alphanumeric1(input)?;
//     //匹配 =
//     let (input, _) = tag("=")(input)?;
//     let parse_escaped_chars = escaped_transform(
//         none_of("\\\""),
//         '\\',
//         alt((value("\\", char('\\')), value("\"", char('\"')))),
//     );

//     let (input, value) = delimited(tag("\""), parse_escaped_chars, tag("\""))(input)?;
//     // Ok((input, (key, value, flag.is_some())))

//     let tag = match flag {
//         Some(f) => {
//             if f.eq("@") {
//                 AttrType::Action
//             } else {
//                 AttrType::Bind
//             }
//         }
//         None => AttrType::Normal,
//     };

//     // let tag = if flag.is_some() {
//     //     AttrType::Action
//     // } else {
//     //     AttrType::Normal
//     // };
//     Ok((
//         input,
//         Tag::Attrs(Attrs {
//             attr_type: tag,
//             key,
//             value,
//         }),
//     ))
// }

// fn tag_close_self(input: &str) -> IResult<&str, &str> {
//     tag("/>")(input)
// }

// // fn tag_close_tag(input: &str) -> IResult<&str, &str> {
// //     //normal close
// //     // let (input,_) = tag(">")(input)?;
// //     fn content(input: &str)->IResult<&str,&str>{
// //         recognize(many0(anychar))(input)
// //     }
// //     dbg!(input);
// //      delimited(
// //         tag(">"),
// //         content,
// //         delimited(tag("</"), alphanumeric1, tag(">")),
// //     )(input)
// // }

// fn tag_close(input: &str)->IResult<&str,&str>{
//    tag(">")(input)
// }

// fn tag_close_tag(input: &str)->IResult<&str,&str>{
//     preceded(
//         tag("</"),
//         alt((
//             recognize(many1(is_a(
//                 "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-",
//             ))),
//             alphanumeric1,
//         )),
//     )(input)

// }

// fn parse_tag_end(input: &str) -> IResult<&str, Tag> {
//     let (input, value) = alt((tag_close_self, tag_close,tag_close_tag))(input)?;
//     Ok((input, Tag::TagEnd(value)))
// }

// enum Tag<'a> {
//     Attrs(Attrs<'a>),
//     Tag(&'a str),
//     TagEnd(&'a str),
// }

// #[derive(Debug)]
// struct Attrs<'a> {
//     attr_type: AttrType,
//     key: &'a str,
//     value: String,
// }
// #[derive(Debug)]
// enum AttrType {
//     /// has @
//     Action,
//     /// bind :
//     Bind,
//     Normal,
// }

// fn parse(input: &str) -> IResult<&str, Vec<Tag>> {
//     // alt((parse_start_tag, parse_self_closing_tag, parse_end_tag))(input)
//     many0(delimited(
//         parse_whitespace,
//         alt((parse_tag_start, parse_attr, parse_tag_end)),
//         parse_whitespace,
//     ))(input)
// }

// fn main() {
//     // let input = r#"
//     // <button value="Hello world" class="button1" @clicked="handle_actions"/>
//     // <text-input value="Click to count" class="input1"></text-input>
//     // <label :value="`Counter: ${counter}`" class="label1"/>
//     // "#;
//     // // let input = r#"<button value="Hello world" class="button1" @clicked="handle_actions"/>"#;
//     // match parse(input) {
//     //     Ok((_s, tags)) => {
//     //         for tag in tags {
//     //             match tag {
//     //                 Tag::Attrs(a) => println!("{:?}", a),
//     //                 Tag::Tag(t) => println!("{:?}", t),
//     //                 Tag::TagEnd(en) => println!("{:?}", en),
//     //             };
//     //         }
//     //     }
//     //     Err(e) => println!("Error: {:?}", e),
//     // }
//     let res = read_to_string("/Users/user/Workspace/others/rsx/simple/template.rsx").unwrap();
//     println!("{:#?}",res);
// }

mod lib;

fn main(){
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
    dbg!(template);
}