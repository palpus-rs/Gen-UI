use std::fmt::Display;

use crate::{STYLE_CLASS, STYLE_ID, STYLE_PESUDO};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum StyleType {
    // class: `.`
    Class,
    // id: `#`
    Id,
    // Pseudo: `::`
    Pseudo,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Class
    }
}

impl StyleType {
    pub fn is_class(&self) -> bool {
        matches!(self, Self::Class)
    }
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id)
    }
    pub fn is_pseudo(&self) -> bool {
        matches!(self, Self::Pseudo)
    }
}

impl From<&str> for StyleType {
    fn from(value: &str) -> Self {
        match value {
            STYLE_CLASS => StyleType::Class,
            STYLE_ID => StyleType::Id,
            STYLE_PESUDO => StyleType::Pseudo,
            _ => panic!("Invalid style"),
        }
    }
}

impl Display for StyleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            StyleType::Class => STYLE_CLASS,
            StyleType::Id => STYLE_ID,
            StyleType::Pseudo => STYLE_PESUDO,
        };
        f.write_str(res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    name: String,
    ty: StyleType,
}

impl Style {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_type(&self) -> &StyleType {
        &self.ty
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            self.get_type().to_string(),
            self.get_name()
        ))
    }
}

#[cfg(test)]
mod test_style {
    use super::StyleType;

    #[test]
    fn style_type() {
        let ty_class = StyleType::Class;
        let ty_id = StyleType::Id;
        let ty_pseudo = StyleType::Pseudo;

        assert_eq!(ty_class, ".".into());
        assert_eq!(ty_id, "#".into());
        assert_eq!(ty_pseudo, "&::".into());
    }
}
