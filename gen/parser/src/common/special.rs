use std::str::FromStr;

use nom::{bytes::complete::tag, sequence::preceded, IResult};

use gen_utils::error::Errors;

use super::parse_closure_body;

pub enum Special {
    MakepadShader,
}

impl Special {
    /// return `(remain, (sign, (name, params, is_style)))`
    pub fn makepad_shader_parser(input: &str) -> IResult<&str, (&str, (&str, &str, Option<bool>))> {
        let (remain, body) = preceded(tag("shader"), parse_closure_body)(input)?;

        Ok((remain, ("()", ("shader", body.trim(), Some(true)))))
    }
}

impl FromStr for Special {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shader" => Ok(Special::MakepadShader),
            _ => Err(Errors::ParseError(format!(
                "this function is not a Special: {}",
                s
            ))),
        }
    }
}
