pub mod inline;
pub mod offline;
pub mod position;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Comments<'a> {
    /// `//`
    Normal(&'a str),
    /// `///`
    Document(&'a str),
    /// `//!`
    File(&'a str),
}

impl<'a> Comments<'a> {
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal(_))
    }
    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document(_))
    }
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
}

impl<'a> Default for Comments<'a> {
   fn default() -> Self {
       Comments::Normal("")
   }
}

impl<'a> From<&'a str> for Comments<'a> {
    fn from(value: &'a str) -> Self {
        Comments::Normal(value)
    }
}
impl<'a> From<(&'a str,&'a str)> for Comments<'a> {
    fn from(value: (&'a str,&'a str)) -> Self {
        match value.0 {
            "//"=>Comments::Normal(value.1),
            "///"=>Comments::Document(value.1),
            "//!"=>Comments::File(value.1),
            _=>panic!("Invalid comment")
        }
    }
}

impl<'a> Display for Comments<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Comments::Normal(n) => format!("// {}",n),
            Comments::Document(d) => format!("/// {}",d),
            Comments::File(f) => format!("//! {}",f),
        };
        f.write_str(res.as_str())
    }
}

#[cfg(test)]
mod test_comments {
    use super::Comments;

    #[test]
    fn display() {
        let c = Comments::Document("hello");
        assert_eq!(c.to_string().as_str(), "/// hello");
    }
}
