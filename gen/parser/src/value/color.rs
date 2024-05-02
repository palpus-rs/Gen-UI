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

// #[derive(Debug, Clone, PartialEq)]
// pub enum Color {
//     /// rgba(0~255, 0~255, 0~255, 0~1.0)
//     Rgba(Rgba),
//     LinearGradient(LinearGradient),
//     RadialGradient(RadialGradient),
// }

// impl Default for Color {
//     fn default() -> Self {
//         Color::Rgba(Default::default())
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct Rgba {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
//     pub alpha: f32,
// }

// impl Default for Rgba {
//     fn default() -> Self {
//         Self {
//             red: Default::default(),
//             green: Default::default(),
//             blue: Default::default(),
//             alpha: 1.0,
//         }
//     }
// }

// impl From<&str> for Rgba {
//     fn from(value: &str) -> Self {
//         let value = value.trim();
//         if value.starts_with("#") {
//             let value = value.trim_start_matches("#");
//             let red = u8::from_str_radix(&value[0..2], 16).unwrap();
//             let green = u8::from_str_radix(&value[2..4], 16).unwrap();
//             let blue = u8::from_str_radix(&value[4..6], 16).unwrap();
//             Self {
//                 red,
//                 green,
//                 blue,
//                 alpha: 1.0,
//             }
//         } else {
//             panic!("Invalid color format: {}", value);
//         }
//     }
// }

// impl Rgba {
//     pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
//         Self {
//             red,
//             green,
//             blue,
//             alpha,
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct LinearGradient {
//     pub deg: f32,
//     pub distribution: Vec<(Rgba, f32)>,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct RadialGradient {
//     pub position: Position,
//     pub distribution: Vec<(Rgba, f32)>,
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Position {
//     /// circle ----
//     Center,
//     /// ellipse ----
//     Left,
//     Right,
//     Top,
//     Bottom,
// }
