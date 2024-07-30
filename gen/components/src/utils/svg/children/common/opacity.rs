#[derive(Debug, Clone, Copy)]
pub struct Opacity(pub f32);

impl Default for Opacity {
    fn default() -> Self {
        Self(1_f32)
    }
}

impl std::str::FromStr for Opacity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let f = s.parse().unwrap();
        if f < 0_f32 || f > 1_f32 {
            return Err(format!("Invalid opacity value: {}", f));
        }
        Ok(Opacity(f))
    }
}

impl std::fmt::Display for Opacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
