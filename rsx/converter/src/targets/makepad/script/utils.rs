use std::collections::HashMap;

use crate::{
    targets::makepad::{
        action::MakepadAction, generate_label_props, model::props_to_string,
        value::MakepadPropValue, BindAction, BindProp, PropRole,
    },
    utils::alphabetic::{camel_to_snake, remove_expr_link, uppercase_title},
};

use super::NodeVariable;

/// Convert `Vec<MakepadAction>` to String
/// build actions
/// ``` rust
/// impl MatchEvent for App{
/// fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){
///     if self.ui.button(id!(button1)).clicked(&actions) {
///         log!("BUTTON CLICKED {}", self.counter);
///         self.counter += 1;
///         let label = self.ui.label(id!(label1));
///         label.set_text_and_redraw(cx,&format!("Counter: {}", self.counter));
///     }
/// }
/// }
/// ```
pub fn fns_to_string(
    name: String,
    fns: &mut Vec<MakepadAction>,
    actions: &Vec<BindAction>,
    binds: Option<&Vec<BindProp>>,
) -> String {
    let mut action_fn = HashMap::new();
    for (tag, id, (action, action_var)) in actions {
        // if mfn.get_name() == actions.
        match fns.iter_mut().find(|f| f.get_name() == action_var) {
            Some(f) => {
                // let _ = action_str.push();

                // let setup = bind_normal();
                action_fn
                    .entry((tag, id))
                    .or_insert_with(Vec::new)
                    .push((action, format!("{} {}", f.to_code(binds), "")));
            }
            None => {}
        }
    }
    let action_str = action_fn
        .into_iter()
        .map(|((tag, id), v)| {
            let tag = camel_to_snake(tag);
            let mut action_str = Vec::new();
            for (action, code) in v {
                action_str.push(format!(
                    "if self.ui.{}(id!({})).{}(&actions) {{ {} }}",
                    &tag, id, action, code
                ));
            }
            action_str.join("\n")
        })
        .collect::<String>();

    format!("impl MatchEvent for {} {{ fn handle_actions(&mut self, cx: &mut Cx, actions:&Actions){{ {} }} }}", name, action_str)
}

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
                // let init = var.init_to_string().unwrap();
                let r = PropRole::try_from((tag, (prop, *var))).unwrap();

                if var.is_mut() {
                    // convert to PropRole and it will get prop_name and prop_value
                    // then get the init value
                    // instance_fields.push((
                    //     var.get_name(),
                    //     remove_expr_link(var.get_ty_str()),
                    //     r.to_normal_value(),
                    //     tag,
                    //     prop,
                    //     id,
                    //     init,
                    // ))
                    // instance_fields.push((var.get_name(), r.to_normal_value(), tag, prop, id))
                    instance_fields.push((var.get_name(), r, tag, id))
                } else {
                    normal_fields.push((r, tag, id))
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
/// `(value, tag, id)`
pub fn build_normal(fields: Vec<(PropRole, &String, &String)>) -> String {
    build_setup(fields)
}

fn build_setup(fields: Vec<(PropRole, &String, &String)>) -> String {
    let mut setup = HashMap::new();
    for (prop, tag, id) in fields {
        let tag = camel_to_snake(tag);
        setup.entry((tag, id)).or_insert_with(Vec::new).push(prop);
    }

    setup
        .into_iter()
        .map(|((tag, id), v)| {
            // let props = v
            //     .into_iter()
            //     .map(|(prop, value)| format!("{}: {}", prop, value))
            //     .collect::<Vec<String>>()
            //     .join(", ");
            let widget_name = uppercase_title(&tag).unwrap();

            let props = props_to_string(&widget_name, &v);
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
fn build_instance(fields: Vec<(&str, PropRole, &String, &String)>) -> (String, String) {
    let mut fields_strs = Vec::new();
    let mut init_strs = Vec::new();
    let mut impls = Vec::new();
    let mut setup = HashMap::new();
    for (name, prop, tag, id) in fields {
        let value = prop.clone().to_normal_value();
        let prop_ty = value.to_makepad_ty();
        let prop_init = value.to_value_code();
        let tag = camel_to_snake(tag);
        fields_strs.push(format!("pub {}: {}", name, &prop_ty));
        // init_strs.push(format!("{}: {}", name, value.to_string()));
        init_strs.push(format!("{}: {}", name, prop_init));
        impls.push(format!(
            "pub fn get_{}(&self) -> &{} {{ &self.{} }}",
            name, &prop_ty, name
        ));
        impls.push(format!(
            "pub fn set_{}(&mut self, {}: {}) {{ self.{} = {} }}",
            name, name, &prop_ty, name, name
        ));
        setup.entry((tag, id)).or_insert_with(Vec::new).push(prop)
    }

    // build setup
    let setup_str = setup
        .into_iter()
        .map(|((tag, id), v)| {
            // let props = v
            //     .into_iter()
            //     .map(|(prop, value)| format!("{}: {}", prop, value))
            //     .collect::<Vec<String>>()
            //     .join(", ");
            // let props = v
            //     .into_iter()
            //     .map(|item| item.to_string())
            //     .collect::<String>();
            let widget_name = uppercase_title(&tag).unwrap();

            let props = props_to_string(&widget_name, &v);
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

// fn build_instance(
//     fields: Vec<(
//         &str,
//         String,
//         MakepadPropValue,
//         &String,
//         &String,
//         &String,
//         String,
//     )>,
// ) -> (String, String) {
//     let mut fields_strs = Vec::new();
//     let mut init_strs = Vec::new();
//     let mut impls = Vec::new();
//     let mut setup = HashMap::new();
//     for (name, _, value, tag, prop, id, _) in fields {
//         let prop_ty = value.to_makepad_ty();
//         let prop_init = value.to_value_code();
//         let tag = camel_to_snake(tag);
//         fields_strs.push(format!("pub {}: {}", name, &prop_ty));
//         // init_strs.push(format!("{}: {}", name, value.to_string()));
//         init_strs.push(format!("{}: {}", name, prop_init));
//         impls.push(format!(
//             "pub fn get_{}(&self) -> &{} {{ &self.{} }}",
//             name, &prop_ty, name
//         ));
//         impls.push(format!(
//             "pub fn set_{}(&mut self, {}: {}) {{ self.{} = {} }}",
//             name, name, &prop_ty, name, name
//         ));
//         setup
//             .entry((tag, id))
//             .or_insert_with(Vec::new)
//             .push((prop, value))
//     }

//     // build setup
//     let setup_str = setup
//         .into_iter()
//         .map(|((tag, id), v)| {
//             let props = v
//                 .into_iter()
//                 .map(|(prop, value)| format!("{}: {}", prop, value))
//                 .collect::<Vec<String>>()
//                 .join(", ");
//             format!(
//                 "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
//                 tag, id, tag, id, tag, id, props
//             )
//         })
//         .collect::<String>();

//     // build struct
//     // build impl new function
//     (format!(
//         "#[derive(Debug, Clone, Default)]\nstruct Instance {{ {} }}\nimpl Instance {{ pub fn new() -> Self {{ Self {{ {} }} }} {} }}",
//         fields_strs.join(", "),
//         init_strs.join(", "),
//         impls.join(" ")
//     ), setup_str )
// }
