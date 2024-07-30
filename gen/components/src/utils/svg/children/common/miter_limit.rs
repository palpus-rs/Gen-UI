#[derive(Debug, Clone, Copy)]
pub struct MiterLimit(pub f32);

impl Default for MiterLimit {
    fn default() -> Self {
        Self(4_f32)
    }
}

impl std::str::FromStr for MiterLimit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let f = s.parse().unwrap();
        Ok(MiterLimit(f))
    }
}

impl std::fmt::Display for MiterLimit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
