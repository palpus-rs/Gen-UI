use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Hex(String),
    Rgb((u8, u8, u8)),
    Rgba((u8, u8, u8, f32)),
}

impl Color {
    pub fn hex(hex: &str) -> Self {
        Color::Hex(String::from(hex))
    }
    pub fn rgb(rgb: (u8, u8, u8)) -> Self {
        Color::Rgb(rgb)
    }
    pub fn rgba(rgba: (u8, u8, u8, f32)) -> Self {
        if rgba.3.gt(&0.0) && rgba.3.lt(&1.0) {
            Color::Rgba(rgba)
        } else {
            panic!("Opacity must be between 0..1, now: {}", rgba.3);
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => f.write_fmt(format_args!("#{}", hex)),
            Color::Rgb((red, green, blue)) => {
                f.write_fmt(format_args!("rgb({}, {}, {})", red, green, blue))
            }
            Color::Rgba((red, green, blue, opacity)) => f.write_fmt(format_args!(
                "rgb({}, {}, {}, {})",
                red, green, blue, opacity
            )),
        }
    }
}
