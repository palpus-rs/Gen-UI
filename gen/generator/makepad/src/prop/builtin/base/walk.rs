use std::fmt::Display;

use super::{DVec2, Margin, Size};

#[derive(Debug, Clone, Default)]
pub struct Walk {
    pub abs_pos: Option<DVec2>,
    pub margin: Option<Margin>,
    pub width: Option<Size>,
    pub height: Option<Size>,
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