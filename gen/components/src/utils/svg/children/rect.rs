use std::fmt::Display;

use super::CommonKVs;

/// The <rect> element is a basic SVG shape that draws rectangles,
/// defined by their position, width, and height. The rectangles may have their corners rounded.
#[derive(Debug, Clone, Default)]
pub struct Rect {
    pub common: CommonKVs,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub rx: Option<f32>,
    pub ry: Option<f32>,
}

impl Rect {
    pub fn from_str(s: &str) -> nom::IResult<&str, Rect> {
        let (s, (common, kvs)) = CommonKVs::from_str(s.trim()).unwrap();

        if kvs.is_none() {
            Ok((
                s,
                Rect {
                    common,
                    x: None,
                    y: None,
                    width: None,
                    height: None,
                    rx: None,
                    ry: None,
                },
            ))
        } else {
            Ok((
                s,
                Rect {
                    common,
                    x: kvs.as_ref().unwrap().get("x").map(|v| v.parse().unwrap()),
                    y: kvs.as_ref().unwrap().get("y").map(|v| v.parse().unwrap()),
                    width: kvs
                        .as_ref()
                        .unwrap()
                        .get("width")
                        .map(|v| v.parse().unwrap()),
                    height: kvs
                        .as_ref()
                        .unwrap()
                        .get("height")
                        .map(|v| v.parse().unwrap()),
                    rx: kvs.as_ref().unwrap().get("rx").map(|v| v.parse().unwrap()),
                    ry: kvs.as_ref().unwrap().get("ry").map(|v| v.parse().unwrap()),
                },
            ))
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\" {} />",
            self.x.as_ref().unwrap_or(&0_f32),
            self.y.as_ref().unwrap_or(&0_f32),
            self.width.as_ref().unwrap_or(&0_f32),
            self.height.as_ref().unwrap_or(&0_f32),
            self.rx.as_ref().unwrap_or(&0_f32),
            self.ry.as_ref().unwrap_or(&0_f32),
            self.common
        )
    }
}
