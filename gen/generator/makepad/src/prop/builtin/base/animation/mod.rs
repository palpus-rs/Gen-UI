mod ease;
mod item;
mod play;

use std::fmt::Display;

pub use ease::*;
use gen_parser::Value;
use gen_utils::error::Errors;
pub use item::*;

use crate::widget::BuiltIn;

#[derive(Debug, Clone)]
pub struct Animation(pub Vec<AnimationItem>);

impl Animation {
    pub fn push(&mut self, item: AnimationItem) {
        self.0.push(item);
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        for item in &self.0 {
            fmt_str.push_str(&item.to_string());
        }
        f.write_str(&fmt_str)
    }
}

impl TryFrom<(&str, &Value, BuiltIn)> for Animation {
    type Error = Errors;

    fn try_from(value: (&str, &Value, BuiltIn)) -> Result<Self, Self::Error> {
        match AnimationItem::try_from(value) {
            Ok(item) => Ok(Animation(vec![item])),
            Err(e) => Err(e),
        }
    }
}
