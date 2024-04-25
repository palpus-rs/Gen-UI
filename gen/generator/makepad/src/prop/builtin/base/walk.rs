use std::fmt::Display;

use gen_converter::error::Errors;
use gen_parser::Value;

use super::{DVec2, Margin, Size};

#[derive(Debug, Clone, Default)]
pub struct Walk {
    pub abs_pos: Option<DVec2>,
    pub margin: Option<Margin>,
    pub width: Option<Size>,
    pub height: Option<Size>,
}

impl Walk {
    pub fn height(&mut self, value: &Value) -> Result<(), Errors> {
        let size = Size::try_from(value)?;
        self.height = Some(size);
        Ok(())
    }
    fn width(&mut self, value: &Value) -> Result<(), Errors> {
        let size = Size::try_from(value)?;
        self.width = Some(size);
        Ok(())
    }
}

impl Display for Walk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut walk = String::new();
        if let Some(abs_pos) = &self.abs_pos {
            walk.push_str(&format!("abs_pos: {}, ", abs_pos));
        }
        if let Some(margin) = &self.margin {
            walk.push_str(&format!("margin: {}, ", margin));
        }
        if let Some(width) = &self.width {
            walk.push_str(&format!("width: {}, ", width));
        }
        if let Some(height) = &self.height {
            walk.push_str(&format!("height: {}, ", height));
        }
        write!(f, "{}", walk)
    }
}
