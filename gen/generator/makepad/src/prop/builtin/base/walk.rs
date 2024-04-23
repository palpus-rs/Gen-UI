use super::{DVec2, Margin, Size};

#[derive(Debug, Clone, Default)]
pub struct Walk {
    pub abs_pos: Option<DVec2>,
    pub margin: Margin,
    pub width: Size,
    pub height: Size,
}
