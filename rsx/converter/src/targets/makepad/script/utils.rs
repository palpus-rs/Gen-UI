use std::collections::HashMap;

use crate::{
    targets::makepad::{value::MakepadPropValue, BindProp, PropRole},
    utils::alphabetic::{camel_to_snake, remove_expr_link},
};

use super::NodeVariable;

/// Convert `Vec<NodeVariable>` to String
/// - mut
/// - immut
/// the design is following:
/// 1. mut variable: if use mut var to bind widget's props,
/// it will generate into Instance struct and auto generate get and set function for each bind prop
/// and a new function for Instance to make sure consistence
/// 2. immutable variable: if use immut var to bind widget's props means
/// this bind just happen in Event::Startup
///
/// RSX will generate a start_up function for bind props
pub fn vars_to_string(name: String, vars: Vec<&NodeVariable>, binds: &Vec<BindProp>) -> String {
    let mut instance_fields = Vec::new();
    let mut normal_fields = Vec::new();
    for (tag, id, (prop, value)) in binds {
        match vars
            .iter()
            .find(|var| var.get_name() == value.get_bind_key())
        {
            Some(var) => {
                let init = var.init_to_string().unwrap();
                let r = PropRole::try_from((tag, (prop, *var))).unwrap();
                if var.is_mut() {
                    // convert to PropRole and it will get prop_name and prop_value
                    // then get the init value

                    instance_fields.push((
                        var.get_name(),
                        remove_expr_link(var.get_ty_str()),
                        r.to_normal_value(),
                        tag,
                        prop,
                        id,
                        init,
                    ))
                } else {
                    normal_fields.push((r.to_normal_value(), tag, prop, id))
                }
            }
            None => {}
        }
    }

    let (instance, mut_setup) = build_instance(instance_fields);
    let immut_setup = build_normal(normal_fields);

    format!(
        "{}\nimpl {}{{ fn start_up(&mut self, cx: &mut Cx){{ self.instance = Instance::new(); {} {} }} }}",
        instance, name, mut_setup, immut_setup
    )
}

/// build normal is aim to add other immuatable properties into start_up function
fn build_normal(fields: Vec<(MakepadPropValue, &String, &String, &String)>) -> String {
    build_setup(fields)
}

fn build_setup(fields: Vec<(MakepadPropValue, &String, &String, &String)>) -> String {
    let mut setup = HashMap::new();
    for (value, tag, prop, id) in fields {
        let tag = camel_to_snake(tag);
        setup
            .entry((tag, id))
            .or_insert_with(Vec::new)
            .push((prop, value));
    }

    setup
        .into_iter()
        .map(|((tag, id), v)| {
            let props = v
                .into_iter()
                .map(|(prop, value)| format!("{}: {}", prop, value))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
                tag, id, tag, id, tag, id, props
            )
        })
        .collect::<String>()
}

/// build an Instance Struct
/// ``` rust
/// let mut wid:String = String::from("Full");
/// ---
/// struct Instance {
///     pub wid: String,
/// }
///
/// impl Instance {
///     pub fn new() -> Self {
///          Self {
///             wid: String::from("Full"),
///         }
///     }
///     pub fn get_wid(&self) -> &String {
///          &self.wid
///     }
///     pub fn set_wid(&mut self, wid: String) {
///          self.wid = wid;
///     }
/// }
///
/// impl App {
///     fn start_up(&self, cx: &mut Cx) {
///         self.instance = Instance::new();
///         let view = self.ui.view(id!(body));
///         view.apply_over_and_redraw(cx, live!{
///             wid: 190,
///         });
///     }
/// }
/// ```
fn build_instance(
    fields: Vec<(
        &str,
        String,
        MakepadPropValue,
        &String,
        &String,
        &String,
        String,
    )>,
) -> (String, String) {
    let mut fields_strs = Vec::new();
    let mut init_strs = Vec::new();
    let mut impls = Vec::new();
    let mut setup = HashMap::new();
    for (name, ty, value, tag, prop, id, init) in fields {
        let tag = camel_to_snake(tag);
        fields_strs.push(format!("pub {}: {}", name, ty));
        // init_strs.push(format!("{}: {}", name, value.to_string()));
        init_strs.push(format!("{}: {}", name, init));
        impls.push(format!(
            "pub fn get_{}(&self) -> &{} {{ &self.{} }}",
            name, ty, name
        ));
        impls.push(format!(
            "pub fn set_{}(&mut self, {}: {}) {{ self.{} = {} }}",
            name, name, ty, name, name
        ));
        setup
            .entry((tag, id))
            .or_insert_with(Vec::new)
            .push((prop, value))
    }

    // build setup
    let setup_str = setup
        .into_iter()
        .map(|((tag, id), v)| {
            let props = v
                .into_iter()
                .map(|(prop, value)| format!("{}: {}", prop, value))
                .collect::<Vec<String>>()
                .join(", ");
            format!(
                "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
                tag, id, tag, id, tag, id, props
            )
        })
        .collect::<String>();

    // build struct
    // build impl new function
    (format!(
        "#[derive(Debug, Clone, Default)]\nstruct Instance {{ {} }}\nimpl Instance {{ pub fn new() -> Self {{ Self {{ {} }} }} {} }}",
        fields_strs.join(", "),
        init_strs.join(", "),
        impls.join(" ")
    ), setup_str )
}
