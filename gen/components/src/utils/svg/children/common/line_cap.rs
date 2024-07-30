use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        LineCap::Butt
    }
}

impl std::str::FromStr for LineCap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "butt" => Ok(LineCap::Butt),
            "round" => Ok(LineCap::Round),
            "square" => Ok(LineCap::Square),
            _ => Err("".to_string()),
        }
    }
}

impl Display for LineCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCap::Butt => write!(f, "butt"),
            LineCap::Round => write!(f, "round"),
            LineCap::Square => write!(f, "square"),
        }
    }
}
