use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use crate::widget::utils::{bool_prop, f64_prop};

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

impl Layout {
    pub fn scroll(&mut self, value: &Value) -> Result<(), Errors> {
        let scroll = DVec2::try_from(value)?;
        self.scroll = Some(scroll);
        Ok(())
    }
    pub fn clip_x(&mut self, value: &Value) -> Result<(), Errors> {
       bool_prop(value, |b|{self.clip_x = Some(b);})
    }
    pub fn clip_y(&mut self, value: &Value) -> Result<(), Errors> {
        bool_prop(value, |b|{self.clip_y = Some(b);})
     }
    pub fn spacing(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f|{self.spacing = Some(f);})
    }
    pub fn line_spacing(&mut self, value: &Value) -> Result<(), Errors> {
        f64_prop(value, |f|{self.line_spacing = Some(f);})
    }
    pub fn padding(&mut self, value: &Value) -> Result<(), Errors> {
        let padding = Padding::try_from(value)?;
        self.padding = Some(padding);
        Ok(())
    }
    pub fn align(&mut self, value: &Value) -> Result<(), Errors> {
        let align = Align::try_from(value)?;
        self.align = Some(align);
        Ok(())
    }
    pub fn flow(&mut self, value: &Value) -> Result<(), Errors> {
        let flow = Flow::try_from(value)?;
        self.flow = Some(flow);
        Ok(())
    }
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