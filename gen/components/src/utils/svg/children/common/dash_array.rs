use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Default)]
pub struct DashArray(pub Vec<f32>);

impl FromStr for DashArray {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s
            .trim()
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<f32>>();

        Ok(DashArray(iter))
    }
}

impl Display for DashArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
