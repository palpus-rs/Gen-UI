mod prop;
mod prop_ptr;

pub use prop::LabelProps;
pub use prop_ptr::LabelPropPtr;

use proc_macro2::TokenTree;

use crate::{prop::{builtin::text, TEXT}, widget::prop_ignore};

pub fn prop(prop_name: &str, value: &str) ->  (String, Vec<TokenTree>) {
    match prop_name {
        TEXT => text(value),
        _ => {
            if !prop_ignore(prop_name) {
                panic!("cannot match prop");
            }
            todo!("unslolved prop")
        }
    }

   
}
