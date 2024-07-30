mod dash_array;
mod dash_offset;
mod line_cap;
mod line_join;
mod miter_limit;
mod opacity;

use std::{collections::HashMap, fmt::Display};

pub use dash_array::DashArray;
pub use dash_offset::DashOffset;
pub use line_cap::LineCap;
pub use line_join::LineJoin;
pub use miter_limit::MiterLimit;
use nom::IResult;
pub use opacity::Opacity;

use crate::utils::svg::parser::parse_properties;

/// Common key-value pairs for SVG elements.
#[derive(Debug, Default, Clone)]
pub struct CommonKVs {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f32>,
    pub stroke_linecap: Option<LineCap>,
    pub stroke_linejoin: Option<LineJoin>,
    pub stroke_dasharray: Option<DashArray>,
    pub stroke_dashoffset: Option<DashOffset>,
    pub stroke_opacity: Option<Opacity>,
    pub stroke_miterlimit: Option<MiterLimit>,
}

impl CommonKVs {
    pub fn from_str(s: &str) -> IResult<&str, (CommonKVs, Option<HashMap<&str, &str>>)> {
        let (s, kvs) = parse_properties(s.trim()).unwrap();
        let mut others = None;
        let mut fill = None;
        let mut stroke = None;
        let mut stroke_width = None;
        let mut stroke_linecap = None;
        let mut stroke_linejoin = None;
        let mut stroke_dasharray = None;
        let mut stroke_dashoffset = None;
        let mut stroke_opacity = None;
        let mut stroke_miterlimit = None;
        let mut x = None;
        let mut y = None;

        for (k, v) in kvs.into_iter() {
            match k {
                "x" => {
                    let _ = x.replace(v.parse().unwrap());
                }
                "y" => {
                    let _ = y.replace(v.parse().unwrap());
                }
                "fill" => {
                    let _ = fill.replace(v.to_string());
                }
                "stroke" => {
                    let _ = stroke.replace(v.to_string());
                }
                "stroke-width" => {
                    let _ = stroke_width.replace(v.parse().unwrap());
                }
                "stroke-linecap" => {
                    let _ = stroke_linecap.replace(v.parse().unwrap());
                }
                "stroke-linejoin" => {
                    let _ = stroke_linejoin.replace(v.parse().unwrap());
                }
                "stroke-dasharray" => {
                    let _ = stroke_dasharray.replace(v.parse().unwrap());
                }
                "stroke-dashoffset" => {
                    let _ = stroke_dashoffset.replace(v.parse().unwrap());
                }
                "stroke-opacity" => {
                    let _ = stroke_opacity.replace(v.parse().unwrap());
                }
                "stroke-miterlimit" => {
                    let _ = stroke_miterlimit.replace(v.parse().unwrap());
                }
                _ => {
                    others.get_or_insert_with(|| HashMap::new()).insert(k, v);
                }
            }
        }

        Ok((
            s,
            (
                CommonKVs {
                    x,
                    y,
                    fill,
                    stroke,
                    stroke_width,
                    stroke_linecap,
                    stroke_linejoin,
                    stroke_dasharray,
                    stroke_dashoffset,
                    stroke_opacity,
                    stroke_miterlimit,
                },
                others,
            ),
        ))
    }
}

impl Display for CommonKVs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if let Some(x) = &self.x {
            s.push_str(&format!("x=\"{}\" ", x));
        }
        if let Some(y) = &self.y {
            s.push_str(&format!("y=\"{}\" ", y));
        }
        if let Some(fill) = &self.fill {
            s.push_str(&format!("fill=\"{}\" ", fill));
        }
        if let Some(stroke) = &self.stroke {
            s.push_str(&format!("stroke=\"{}\" ", stroke));
        }
        if let Some(stroke_width) = &self.stroke_width {
            s.push_str(&format!("stroke-width=\"{}\" ", stroke_width));
        }
        if let Some(stroke_linecap) = &self.stroke_linecap {
            s.push_str(&format!("stroke-linecap=\"{}\" ", stroke_linecap));
        }
        if let Some(stroke_linejoin) = &self.stroke_linejoin {
            s.push_str(&format!("stroke-linejoin=\"{}\" ", stroke_linejoin));
        }
        if let Some(stroke_dasharray) = &self.stroke_dasharray {
            s.push_str(&format!("stroke-dasharray=\"{}\" ", stroke_dasharray));
        }
        if let Some(stroke_dashoffset) = &self.stroke_dashoffset {
            s.push_str(&format!("stroke-dashoffset=\"{}\" ", stroke_dashoffset));
        }
        if let Some(stroke_opacity) = &self.stroke_opacity {
            s.push_str(&format!("stroke-opacity=\"{}\" ", stroke_opacity));
        }
        if let Some(stroke_miterlimit) = &self.stroke_miterlimit {
            s.push_str(&format!("stroke-miterlimit=\"{}\" ", stroke_miterlimit));
        }
        write!(f, "{}", s)
    }
}
