use std::fmt::Display;

use crate::utils::svg::value::Auto;

use super::CommonKVs;
/// The <ellipse> element is an SVG basic shape,
/// used to create ellipses based on a center coordinate, and both their x and y radius.
#[derive(Debug, Clone)]
pub struct Ellipse {
    pub common: CommonKVs,
    /// The x position of the center of the ellipse. Value type: <length>|<percentage> ; Default value: 0;
    pub cx: Option<f32>,
    /// The y position of the center of the ellipse. Value type: <length>|<percentage> ; Default value: 0;
    pub cy: Option<f32>,
    /// The radius of the ellipse on the x axis. Value type: auto|<length>|<percentage> ; Default value: auto
    pub rx: Auto<f32>,
    /// The radius of the ellipse on the y axis. Value type: auto|<length>|<percentage> ; Default value: auto;
    pub ry: Auto<f32>,
}

impl Ellipse {
    pub fn from_str(s: &str) -> nom::IResult<&str, Ellipse> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((
                s,
                Ellipse {
                    common,
                    cx: None,
                    cy: None,
                    rx: Auto(None),
                    ry: Auto(None),
                },
            ))
        } else {
            let cx = kvs.as_ref().unwrap().get("cx").map(|v| v.parse().unwrap());
            let cy = kvs.as_ref().unwrap().get("cy").map(|v| v.parse().unwrap());
            let rx = Auto::get_from_map(&kvs.as_ref().unwrap(), "rx");
            let ry = Auto::get_from_map(&kvs.as_ref().unwrap(), "ry");

            Ok((
                s,
                Ellipse {
                    common,
                    cx,
                    cy,
                    rx,
                    ry,
                },
            ))
        }
    }
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {} />",
            self.cx.as_ref().unwrap_or(&0_f32),
            self.cy.as_ref().unwrap_or(&0_f32),
            self.rx,
            self.ry,
            self.common
        )
    }
}
