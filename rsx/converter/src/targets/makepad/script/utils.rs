use proc_macro2::TokenTree;

use crate::targets::makepad::{value::MakepadPropValue, BindProp, PropRole};

use super::NodeVariable;

/// Convert Vec<NodeVariable> to String
/// - mut ->  
/// - immut
/// ``` rust
/// let mut wid:String = String::from("Full");
/// ---
/// struct Instance {
///     wid: String,
/// }
///
/// impl Instance {
///     fn new() -> Self {
///          Self {
///             wid: String::from("Full"),
///         }
///     }
/// }
/// ```
pub fn vars_to_string(vars: Vec<&NodeVariable>, binds: &Vec<BindProp>) -> String {
    let mut instance_fields = Vec::new();
    for (tag, (prop, value)) in binds {
        match vars
            .iter()
            .find(|var| var.get_name() == value.get_bind_key())
        {
            Some(var) => {
                if var.is_mut() {
                    // convert to PropRole and it will get prop_name and prop_value
                    // then get the init value
                    let r = PropRole::try_from((tag, (prop, *var))).unwrap();

                    instance_fields.push((var.get_name(), var.get_ty_str(), r.to_normal_value()))
                }
            }
            None => {}
        }
    }

    let instance_str = build_instance(instance_fields);

    String::new()
}

fn build_instance(fields: Vec<(&str, String, MakepadPropValue)>) -> String {
    let mut fields_str = String::new();
    for (name, ty, value) in fields {
        fields_str.push_str(&format!("{}: {},\n", name, ty));
    }
}
