use crate::{COMMENT_DOCUMENT, COMMENT_FILE, COMMENT_NROMAL};

use crate::template::TemplateASTNode;

use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::{
    bytes::complete::{tag, take_until},
    combinator::recognize,
    sequence::delimited,
    IResult,
};

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
