use std::collections::HashMap;

use crate::error::Errors;

use super::PropRole;

type StyleProps = HashMap<String,Option<PropRole>>;

pub fn style_class()->Result<StyleProps,Errors>{
    
}

pub fn style_id()->Result<StyleProps,Errors>{}