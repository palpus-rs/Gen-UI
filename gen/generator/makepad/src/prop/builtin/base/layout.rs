use super::{Align, DVec2, Flow, Padding};

#[derive(Debug,Clone)]
pub struct Layout {
   pub scroll: DVec2,
   pub clip_x: bool,
   pub clip_y: bool,
   pub padding: Padding,
   pub align: Align,
   pub flow: Flow,
   pub spacing: f64,
   pub line_spacing: f64
}