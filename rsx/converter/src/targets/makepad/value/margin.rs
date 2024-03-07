use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub struct Margin {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Display for Margin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{top: {}, right: {}, bottom: {}, left: {}}}",
            self.top, self.right, self.bottom, self.left
        ))
    }
}
