use std::fmt::Display;

use nom::IResult;

use super::CommonKVs;

/// The <line> element is an SVG basic shape used to create a line connecting two points.
#[derive(Debug, Clone, Default)]
pub struct Line{
    pub common: CommonKVs,
    /// Defines the x-axis coordinate of the line starting point. Value type: <length>|<percentage>|<number> ; Default value: 0; 
    pub x1: Option<f32>,
    /// Defines the x-axis coordinate of the line ending point. Value type: <length>|<percentage>|<number> ; Default value: 0;
    pub y1: Option<f32>,
    /// Defines the y-axis coordinate of the line starting point. Value type: <length>|<percentage>|<number> ; Default value: 0; 
    pub x2: Option<f32>,
    /// Defines the y-axis coordinate of the line ending point. Value type: <length>|<percentage>|<number> ; Default value: 0;
    pub y2: Option<f32>,
}

impl Line {
    pub fn from_str(s: &str) -> IResult<&str, Line> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((
                s,
                Line {
                    common,
                    x1: None,
                    y1: None,
                    x2: None,
                    y2: None,
                },
            ))
        } else {
            Ok((
                s,
                Line {
                    common,
                    x1: kvs.as_ref().unwrap().get("x1").map(|v| v.parse().unwrap()),
                    y1: kvs.as_ref().unwrap().get("y1").map(|v| v.parse().unwrap()),
                    x2: kvs.as_ref().unwrap().get("x2").map(|v| v.parse().unwrap()),
                    y2: kvs.as_ref().unwrap().get("y2").map(|v| v.parse().unwrap()),
                },
            ))
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" {} />",
            self.x1.as_ref().unwrap_or(&0_f32),
            self.y1.as_ref().unwrap_or(&0_f32),
            self.x2.as_ref().unwrap_or(&0_f32),
            self.y2.as_ref().unwrap_or(&0_f32),
            self.common
        )
    }
}