use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Hash,Eq)]
pub enum KeyWords{
    /// :props
    Props,
    /// id
    Id,
    /// class
    Class,
    /// inherits
    Inherits,
    Actions_Macro,
}

impl Display for KeyWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self{
            KeyWords::Props => "props",
            KeyWords::Id =>"id",
            KeyWords::Class => "class",
            KeyWords::Inherits =>"inherits",
            KeyWords::Actions_Macro => "actions!",  
        })
    }
}