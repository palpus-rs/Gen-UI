use std::{borrow::Cow, collections::HashMap};

use gen_parser::{PropsKey, Value};

/// also name ConvertStyle
/// in gen-ui no difference between style and props
/// so we use the same struct to represent them
/// `<prop_name, HashMap<prop, value>>`
pub type ConvertProp<'a> = HashMap<Cow<'a, str>, Cow<'a, HashMap<PropsKey, Value>>>;
// /// `(tag_name, id, (prop_name, prop_value))`
// pub type BindProp = (String, String, (String, MakepadPropValue));