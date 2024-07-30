use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum LineJoin {
    Arcs,
    Miter,
    Round,
    Bevel,
    MiterClip,
}

impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter
    }
}

impl std::str::FromStr for LineJoin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arcs" => Ok(LineJoin::Arcs),
            "miter" => Ok(LineJoin::Miter),
            "round" => Ok(LineJoin::Round),
            "bevel" => Ok(LineJoin::Bevel),
            "miter-clip" => Ok(LineJoin::MiterClip),
            _ => Err("".to_string()),
        }
    }
}

impl Display for LineJoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineJoin::Arcs => write!(f, "arcs"),
            LineJoin::Miter => write!(f, "miter"),
            LineJoin::Round => write!(f, "round"),
            LineJoin::Bevel => write!(f, "bevel"),
            LineJoin::MiterClip => write!(f, "miter-clip"),
        }
    }
}