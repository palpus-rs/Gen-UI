use std::{borrow::Cow, collections::HashMap};

use gen_parser::{PropsKey, Value};

/// also name ConvertStyle
/// in gen-ui no difference between style and props
/// so we use the same struct to represent them
/// `<prop_name, HashMap<prop, value>>`
pub type ConvertProp = HashMap<String, HashMap<PropsKey, Value>>;
// /// `(tag_name, id, (prop_name, prop_value))`
// pub type BindProp = (String, String, (String, MakepadPropValue));

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Props {
    normal: Option<Vec<ConvertProp>>,
    bind: Option<Vec<ConvertProp>>,
}


