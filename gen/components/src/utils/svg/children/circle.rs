use core::panic;
use std::fmt::Display;

use nom::IResult;

use super::CommonKVs;

/// The <circle> SVG element is an SVG basic shape,
/// used to draw circles based on a center point and a radius.
#[derive(Debug, Clone, Default)]
pub struct Circle {
    pub common: CommonKVs,
    /// The x-axis coordinate of the center of the circle.
    /// Value type: <length>|<percentage> ; Default value: 0;
    pub cx: Option<f32>,
    /// The y-axis coordinate of the center of the circle.
    /// Value type: <length>|<percentage> ; Default value: 0;
    pub cy: Option<f32>,
    /// The radius of the circle. A value lower or equal to zero disables rendering of the circle. Value type: <length>|<percentage> ; Default value: 0;
    pub r: Option<f32>,
}

impl Circle {
    pub fn from_str(s: &str) -> IResult<&str, Circle> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((
                s,
                Circle {
                    common,
                    cx: None,
                    cy: None,
                    r: None,
                },
            ))
        } else {
            Ok((
                s,
                Circle {
                    common,
                    cx: kvs.as_ref().unwrap().get("cx").map(|v| v.parse().unwrap()),
                    cy: kvs.as_ref().unwrap().get("cy").map(|v| v.parse().unwrap()),
                    r: kvs.as_ref().unwrap().get("r").map(|v| v.parse().unwrap()),
                },
            ))
        }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" />",
            self.cx.as_ref().unwrap_or(&0_f32),
            self.cy.as_ref().unwrap_or(&0_f32),
            self.r.as_ref().unwrap_or(&0_f32)
        )
    }
}
