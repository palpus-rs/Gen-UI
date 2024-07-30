#[derive(Debug, Clone, Copy)]
pub struct DashOffset(pub f32);

impl Default for DashOffset {
    fn default() -> Self {
        Self(0_f32)
    }
}

impl std::str::FromStr for DashOffset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let f = s.parse().unwrap();
        Ok(DashOffset(f))
    }
    
}

impl std::fmt::Display for DashOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}