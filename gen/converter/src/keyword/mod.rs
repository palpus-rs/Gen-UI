use std::fmt::Display;

use gen_parser::Value;

const PROPS: &str = "props";
const ID: &str = "id";
const CLASS: &str = "class";
const INHERITS: &str = "inherits";
const ACTIONS_MACRO: &str = "actions!";
const IF: &str = "if";
const ELSE_IF: &str = "else if";
const ELSE: &str = "else";
const FOR: &str = "for";

/// The key words in gen-ui template
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum KeyWords {
    /// id
    Id,
    /// class
    Class,
    /// inherits
    Inherits,
    /// for
    For,
    /// if
    If,
    /// else if
    ElseIf,
    /// else
    Else,
    Actions_Macro,
}

impl KeyWords {
    // pub fn value_prop<E,P>(&self, value: &Value, model: &mut TemplateModel<E,P>) -> ()
    // where E: Event, P: Prop {
    //     match self {
    //         KeyWords::Id => {
    //             // id只能是单个String或Unknown
    //             // if let Some(id) = value.is_unknown_and_get() {
    //             //     let _ = model.set_special(id);
    //             // } else {
    //             //     value.is_string_and_get().unwrap_or_else(|s| {
    //             //         let _ = model.set_special(s);
    //             //     });
    //             // }
    //             string_unknown(value, |id| {
    //                 model.set_special(id);
    //             });
    //         }
    //         KeyWords::Class => {
    //             // class没有限制，可以是String,Unknown,绑定
    //         }
    //         KeyWords::Inherits => {
    //             // inherits只能是单个String或Unknown
    //             string_unknown(value, |inherits| model.set_inherit(inherits));
    //         }
    //         _ => panic!("KeyWord can not use in Template prop"),
    //     }
    // }
}
impl Display for KeyWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            KeyWords::Id => ID,
            KeyWords::Class => CLASS,
            KeyWords::Inherits => INHERITS,
            KeyWords::Actions_Macro => ACTIONS_MACRO,
            KeyWords::For => FOR,
            KeyWords::If => IF,
            KeyWords::ElseIf => ELSE_IF,
            KeyWords::Else => ELSE,
        })
    }
}

impl TryFrom<&str> for KeyWords {
    type Error = gen_utils::error::Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ID => Ok(KeyWords::Id),
            CLASS => Ok(KeyWords::Class),
            INHERITS => Ok(KeyWords::Inherits),
            ACTIONS_MACRO => Ok(KeyWords::Actions_Macro),
            FOR => Ok(KeyWords::For),
            IF => Ok(KeyWords::If),
            ELSE_IF => Ok(KeyWords::ElseIf),
            ELSE => Ok(KeyWords::Else),
            _ => Err(gen_utils::error::Errors::MissMatchKeyWord),
        }
    }
}

fn string_unknown<F>(value: &Value, f: F) -> ()
where
    F: FnOnce(&str) -> (),
{
    if let Some(id) = value.is_unknown_and_get() {
        let _ = f(id);
    } else {
        value.is_string_and_get().map(|id| {
            let _ = f(id);
        });
    }
}
