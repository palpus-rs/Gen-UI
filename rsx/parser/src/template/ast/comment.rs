use std::fmt::{write, Display};

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Comments {
    /// `//`
    Normal,
    /// `///`
    Document,
    /// `//!`
    File,
}

impl Comments {
    pub fn is_normal(&self) -> bool {
        matches!(self,Self::Normal)
    }
    pub fn is_document(&self) -> bool {
        matches!(self,Self::Document)
    }
    pub fn is_file(&self) -> bool {
        matches!(self,Self::File)
    }
}

impl Default for Comments {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<&str> for Comments {
    fn from(value: &str) -> Self {
        match value {
            "//" => Comments::Normal,
            "///" => Comments::Document,
            "//!" => Comments::File,
            _=> panic!("Invalid comment")
        }
    }
}

impl Display for Comments{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Comments::Normal => "normal",
            Comments::Document => "document",
            Comments::File => "file",
        };
        f.write_str(res)
    }
}

#[cfg(test)]
mod test_comments {
    use super::Comments;

    #[test]
    fn display(){
        let c = Comments::Document;
        assert_eq!(c.to_string().as_str(), "document");
    }
}