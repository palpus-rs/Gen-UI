mod circle;
mod common;
mod ellipse;
mod line;
mod path;
mod rect;

use std::fmt::Display;
use std::str::FromStr;

pub use circle::*;
pub use common::*;
pub use ellipse::*;
pub use line::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::{sequence::preceded, IResult};
pub use path::*;
pub use rect::*;

use super::parser::trim;

#[derive(Debug)]
pub enum Child {
    Circle(Circle),
    Ellipse(Ellipse),
    Line(Line),
    Path(Path),
    Rect(Rect),
    // unsupported now ----------------------
    // Text,
    // TextPath,
    // Tspan,
    // Tref,
    // Polygon,
    // Polyline,
}

impl Child {
    fn which(s: &str) -> IResult<&str, Child> {
        let (s, name) = preceded(
            trim(tag("<")),
            alt((
                tag("path"),
                tag("circle"),
                tag("rect"),
                tag("line"),
                tag("ellipse"),
            )),
        )(s)?;

        Ok((s, name.parse().unwrap()))
    }
    pub fn parser(s: &str) -> IResult<&str, Vec<Child>> {
        fn single(s: &str) -> IResult<&str, Child> {
            let (s, child) = Child::which(s)?;

            let (s, child) = match child {
                Child::Path(_) => {
                    let (s, path) = Path::from_str(s)?;
                    (s, Child::Path(path))
                }
                Child::Circle(_) => {
                    let (s, circle) = Circle::from_str(s)?;
                    (s, Child::Circle(circle))
                }
                Child::Rect(_) => {
                    let (s, rect) = Rect::from_str(s)?;
                    (s, Child::Rect(rect))
                }
                Child::Line(_) => {
                    let (s, line) = Line::from_str(s)?;
                    (s, Child::Line(line))
                }
                Child::Ellipse(_) => {
                    let (s, ellipse) = Ellipse::from_str(s)?;
                    (s, Child::Ellipse(ellipse))
                }
            };
            let (s, _) = trim(tag("/"))(s)?;
            let (s, _) = trim(tag(">"))(s)?;
            Ok((s, child))
        }

        many0(single)(s)
    }
}

impl FromStr for Child {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "circle" => Ok(Child::Circle(Circle::default())),
            "ellipse" => Ok(Child::Ellipse(Ellipse::default())),
            "line" => Ok(Child::Line(Line::default())),
            "path" => Ok(Child::Path(Path::default())),
            "rect" => Ok(Child::Rect(Rect::default())),
            _ => Err(format!("Unsupported children: {}", s)),
        }
    }
}

impl Display for Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Child::Circle(c) => write!(f, "{}", c),
            Child::Ellipse(e) => write!(f, "{}", e),
            Child::Line(l) => write!(f, "{}", l),
            Child::Path(p) => write!(f, "{}", p),
            Child::Rect(r) => write!(f, "{}", r),
        }
    }
}
