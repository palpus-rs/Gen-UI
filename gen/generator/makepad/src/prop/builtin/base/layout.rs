use std::fmt::Display;

use super::{Align, DVec2, Flow, Padding};

#[derive(Debug, Clone, Default)]
pub struct Layout {
    pub scroll: Option<DVec2>,
    pub clip_x: Option<bool>,
    pub clip_y: Option<bool>,
    pub padding: Option<Padding>,
    pub align: Option<Align>,
    pub flow: Option<Flow>,
    pub spacing: Option<f64>,
    pub line_spacing: Option<f64>,
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut layout = String::new();
        if let Some(scroll) = &self.scroll {
            layout.push_str(&format!("scroll: {}, ", scroll));
        }
        if let Some(clip_x) = &self.clip_x {
            layout.push_str(&format!("clip_x: {}, ", clip_x));
        }
        if let Some(clip_y) = &self.clip_y {
            layout.push_str(&format!("clip_y: {}, ", clip_y));
        }
        if let Some(padding) = &self.padding {
            layout.push_str(&format!("padding: {}, ", padding));
        }
        if let Some(align) = &self.align {
            layout.push_str(&format!("align: {}, ", align));
        }
        if let Some(flow) = &self.flow {
            layout.push_str(&format!("flow: {}, ", flow));
        }
        if let Some(spacing) = &self.spacing {
            layout.push_str(&format!("spacing: {}, ", spacing));
        }
        if let Some(line_spacing) = &self.line_spacing {
            layout.push_str(&format!("line_spacing: {}, ", line_spacing));
        }
        write!(f, "{}", layout)
    }
}

#[cfg(test)]
mod test_layout{
    
}