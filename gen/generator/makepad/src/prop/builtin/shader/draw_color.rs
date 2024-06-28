use std::fmt::Display;

use gen_parser::Value;
use gen_utils::error::Errors;

use super::draw_quad::DrawQuad;

// use super::MapValue;

#[derive(Debug, Clone, Default)]
pub struct DrawColor {
    pub color: Option<String>,
    pub draw_super: DrawQuad,
}

impl TryFrom<&Value> for DrawColor {
    type Error = Errors;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let (quad, hex_color) = DrawQuad::try_from_back(value)?;
        let mut draw_color = DrawColor::default();
        // exist color
        draw_color.color = hex_color.map(|hex| hex.0);
        draw_color.draw_super = quad;

        Ok(draw_color)
    }
}

impl Display for DrawColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(color) = &self.color {
            f.write_fmt(format_args!("color: {}", color))
        } else {
            self.draw_super.fmt(f)
        }
    }
}
