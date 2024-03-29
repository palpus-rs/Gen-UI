use std::fmt::Display;

use gen_parser::Value;

use crate::model::Model;

const PROPS: &str = "props";
const ID: &str = "id";
const CLASS: &str = "class";
const INHERITS: &str = "inherits";
const ACTIONS_MACRO: &str = "actions!";

/// The key words in gen-ui template
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum KeyWords {
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

impl KeyWords {
    pub fn value_prop(&self, value: &Value, model: &mut Model) -> () {
        match self {
            KeyWords::Props => {
                // props只能是绑定的
            }
            KeyWords::Id => {
                // id只能是单个String或Unknown
                if let Some(id) = value.is_unknown_and_get() {
                    let _ = model.set_special(id);
                }
            }
            KeyWords::Class => {
                // class没有限制，可以是String,Unknown,绑定
            }
            KeyWords::Inherits => {
                // inherits只能是单个String或Unknown
            }
            _ => panic!("KeyWord can not use in Template prop"),
        }
    }
}
impl Display for KeyWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            KeyWords::Props => PROPS,
            KeyWords::Id => ID,
            KeyWords::Class => CLASS,
            KeyWords::Inherits => INHERITS,
            KeyWords::Actions_Macro => ACTIONS_MACRO,
        })
    }
}

impl TryFrom<&str> for KeyWords {
    type Error = crate::error::Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            PROPS => Ok(KeyWords::Props),
            ID => Ok(KeyWords::Id),
            CLASS => Ok(KeyWords::Class),
            INHERITS => Ok(KeyWords::Inherits),
            ACTIONS_MACRO => Ok(KeyWords::Actions_Macro),
            _ => Err(crate::error::Errors::MissMatchKeyWord),
        }
    }
}
