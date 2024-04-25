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
        if let Some(s) = value.is_unknown_and_get() {
            let _ = self.height.replace(s.try_into()?);
            Ok(())
        } else if let Some(d) = value.is_double_and_get() {
            let _ = self.height.replace(d.into());
            Ok(())
        } else if let Some(d) = value.is_float_and_get() {
            let _ = self.height.replace((d as f64).into());
            Ok(())
        } else {
            value
                .is_string_and_get()
                .map(|s| {
                    let _ = self.height.replace(s.try_into()?);
                    Ok(())
                })
                .unwrap_or_else(|| {
                    Err(Errors::PropConvertFail(format!(
                        "{} can not convert to height",
                        value
                    )))
                })
        }
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
