use std::collections::HashMap;

use gen_parser::{PropsKey, Value};
use gen_traits::prop::Prop;

/// also name ConvertStyle
/// in gen-ui no difference between style and props
/// so we use the same struct to represent them
/// `<id|class, HashMap<prop, value>>`
pub type ConvertStyle = HashMap<String, HashMap<PropsKey, Value>>;

#[derive(Debug, gen_macros::Prop, Default, Clone)]
pub struct NoProps {}
