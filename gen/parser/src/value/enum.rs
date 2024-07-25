use std::fmt::Display;

/// # Enum Value
/// parse like: `TextAlign.Center.place`-> `Enum{name: TextAlign, field_chain: [Center, place]}`
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    /// enum name
    name: String,
    /// enum fields
    /// key: field name
    /// value: field value (if has)
    field_chain: Vec<String>,
}

impl From<(&str, Vec<&str>)> for Enum {
    fn from(value: (&str, Vec<&str>)) -> Self {
        let name = value.0.to_string();

        if value.1.len() == 0 {
            panic!("Enum must have at least one level field");
        } else {
            Enum {
                name,
                field_chain: value.1.iter().map(|s| s.to_string()).collect(),
            }
        }
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_chain = self.field_chain.join(".");
        f.write_fmt(format_args!("{}.{}", self.name, field_chain))
    }
}
