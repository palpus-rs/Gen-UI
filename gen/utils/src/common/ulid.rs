use std::fmt::Display;

/// # Ulid
/// use ulid crate to generate ulid
#[derive(Clone, Debug, PartialEq)]
pub struct Ulid(pub String);

impl Ulid {
    pub fn new() -> Self {
        Ulid(ulid())
    }
}

impl Display for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn ulid() -> String {
    ulid::Ulid::new().to_string()
}
