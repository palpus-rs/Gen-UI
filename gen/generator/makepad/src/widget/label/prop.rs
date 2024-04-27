use crate::prop::builtin::{Align, Padding, Walk};

#[derive(Debug, Clone, Default)]
pub struct LabelProps {
    pub draw_text: Option<Draw>,
    pub walk: Option<Walk>,
    pub align: Option<Align>,
    pub padding: Option<Padding>,
    pub text: Option<String>,
}
