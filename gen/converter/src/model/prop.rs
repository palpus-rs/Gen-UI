use std::{borrow::Cow, collections::HashMap};

use gen_parser::{PropsKey, Value};
use gen_traits::prop::Prop;

/// also name ConvertStyle
/// in gen-ui no difference between style and props
/// so we use the same struct to represent them
/// `<id|class, HashMap<prop, value>>`
pub type ConvertStyle = HashMap<String, HashMap<PropsKey, Value>>;
pub type ConvertProp = HashMap<PropsKey,Value>;
// /// `(tag_name, id, (prop_name, prop_value))`
// pub type BindProp = (String, String, (String, MakepadPropValue));

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Props {
    normal: Option<Vec<ConvertProp>>,
    bind: Option<Vec<ConvertProp>>,
}

#[derive(Debug, Clone, gen_macros::Prop)]
pub struct NoProps{}
// #[derive(Debug, PartialEq, Clone)]
// pub enum PropRole {
//     Normal(String, MakepadPropValue),
//     Bind(String, MakepadPropValue),
//     Function(String, MakepadPropValue),
//     // this means: the current prop is id or class which can link to style properties  (class)
//     Context(Vec<String>),
//     // same as Context, but only have one (id)
//     Special(String),
//     Component(Widgets),
// }