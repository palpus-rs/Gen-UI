use std::collections::HashMap;

use parser::{PropsKey, Value};

use crate::error::Errors;

use super::{value::MakepadPropValue, PropRole};

pub type StyleProps = HashMap<String, Vec<PropRole>>;

// expand props directly
// then when it in the widget -> convert
// pub fn style_props_expand(props: &HashMap<PropsKey, Value>) -> Vec<> {
//     for (k, v) in props {
//         match k.ty() {
//             parser::PropertyKeyType::Normal => ,
//             parser::PropertyKeyType::Bind => todo!(),
//             parser::PropertyKeyType::Function => todo!(),
//         }
//     }
// }
//
// pub fn style_id() -> Result<StyleProps, Errors> {}
