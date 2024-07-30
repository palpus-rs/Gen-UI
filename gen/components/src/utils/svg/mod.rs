pub mod children;
pub mod encode;
pub mod header;
pub mod macros;
pub mod parser;
pub mod value;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use children::Child;
use header::Header;
use nom::bytes::complete::{tag, take_until1};
use nom::character::complete::alpha1;
use nom::sequence::{delimited, pair, preceded, terminated};
use parser::{parse_properties, trim};
use value::Auto;

#[derive(Debug, Default)]
pub struct Svg {
    pub header: Option<Header>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub height: Auto<f32>,
    pub width: Auto<f32>,
    pub view_box: Option<(u32, u32, u32, u32)>,
    pub fill: String,
    pub xmlns: String,
    pub children: Vec<Child>,
}

impl FromStr for Svg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut svg = Svg::default();
        // parse header -----------------------------------------------------------------------
        let (s, header) = if let Ok((s, header)) = Header::from_str(s) {
            (s, Some(header))
        } else {
            (s, None)
        };
        svg.header = header;
        // parse svg tag -----------------------------------------------------------------------
        let (s, props_str) = delimited(
            preceded(trim(tag("<")), tag("svg")),
            trim(take_until1(">")),
            trim(tag(">")),
        )(s)
        .unwrap();

        let kvs: Option<HashMap<&str, &str>> = if !props_str.is_empty() {
            let (remain, props) = parse_properties(props_str).unwrap();
            if !remain.is_empty() {
                return Err("Invalid svg tag props!".to_string());
            }
            Some(HashMap::from_iter(props.into_iter()))
        } else {
            None
        };

        if kvs.is_some() {
            svg.fill = kvs
                .as_ref()
                .unwrap()
                .get("fill")
                .unwrap_or(&"none")
                .to_string();
            svg.xmlns = kvs
                .as_ref()
                .unwrap()
                .get("xmlns")
                .unwrap_or(&"http://www.w3.org/2000/svg")
                .to_string();
            svg.x = kvs.as_ref().unwrap().get("x").map(|x| x.parse().unwrap());
            svg.y = kvs.as_ref().unwrap().get("y").map(|y| y.parse().unwrap());
            svg.height = Auto::get_from_map(&kvs.as_ref().unwrap(), "height");
            svg.width = Auto::get_from_map(&kvs.as_ref().unwrap(), "width");
            svg.view_box = kvs.as_ref().unwrap().get("viewBox").map(|view_box| {
                let mut iter = view_box.split_whitespace();
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                let width = iter.next().unwrap().parse().unwrap();
                let height = iter.next().unwrap().parse().unwrap();
                (x, y, width, height)
            });
        }

        // children ----------------------------------------------------------------------------
        let (s, children) = Child::parser(s).unwrap();
        svg.children = children;
        let (s, _) = delimited(
            preceded(trim(tag("<")), tag("/")),
            trim(tag("svg")),
            trim(tag(">")),
        )(s)
        .unwrap();
        if !s.is_empty() {
            return Err(format!("Invalid svg tag: {}", s));
        }
        Ok(svg)
    }
}

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(header) = &self.header {
            s.push_str(&header.to_string());
        }
        s.push_str("<svg ");
        if !self.fill.is_empty() {
            s.push_str(&format!("fill=\"{}\" ", self.fill));
        }
        if !self.xmlns.is_empty() {
            s.push_str(&format!("xmlns=\"{}\" ", self.xmlns));
        }
        if let Some(x) = self.x {
            s.push_str(&format!("x=\"{}\" ", x));
        }
        if let Some(y) = self.y {
            s.push_str(&format!("y=\"{}\" ", y));
        }
        if let Some(view_box) = self.view_box {
            s.push_str(&format!(
                "viewBox=\"{} {} {} {}\" ",
                view_box.0, view_box.1, view_box.2, view_box.3
            ));
        }
        s.push_str(&format!("height=\"{}\" ", self.height));
        s.push_str(&format!("width=\"{}\" ", self.width));

        s.push_str(">");
        for child in &self.children {
            s.push_str(&child.to_string());
        }
        s.push_str("</svg>");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test_svg {
    use std::fs::read_to_string;

    #[test]
    fn test_svg() {
        use super::Svg;
        let s =
            read_to_string("E:/Rust/try/makepad/Gen-UI/gen/components/resources/icons/alarm.svg")
                .unwrap();
        let svg: Svg = s.parse().unwrap();
        dbg!(svg.to_string());
    }
}
