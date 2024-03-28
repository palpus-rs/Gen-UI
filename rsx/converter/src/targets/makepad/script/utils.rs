use std::collections::HashMap;

use proc_macro2::{Punct, Spacing, Span, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{Expr, Ident, ItemStruct, Meta};

use crate::{
    context::{LEFT_HOLDER, RIGHT_HOLDER},
    targets::makepad::{
        action::MakepadAction, handler::MakepadFieldConverter, model::props_to_string, BindAction,
        BindProp, MakepadWidgetActions, PropRole, Widgets,
    },
    utils::{
        alphabetic::{camel_to_snake, snake_to_camel},
        macros::build_attr_macro,
    },
};

use super::{
    handler::{build_handle_actions, build_handle_startup},
    NodeVariable,
};

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
    fns: &mut Vec<MakepadAction>,
    actions: &Vec<BindAction>,
    binds: Option<&Vec<BindProp>>,
) -> String {
    let action_str = build_match_handle_event(
        fns,
        actions,
        binds,
        |actions, tag, id, action_code_list| -> String {
            for (action, code) in action_code_list {
                actions.push(format!(
                    "if self.ui.{}(id!({})).{}(&actions) {{ {} }}",
                    tag, id, action, code
                ));
            }
            actions.join(" ")
        },
    );

    build_handle_actions(&action_str)
}

/// ## example
/// for action in cx.capture_actions(|cx| self.button(id!(bb)).handle_event(cx, event, scope)) {
///     match action.as_widget_action().cast(){
///         ButtonAction::Clicked => { log!("Button clicked");}
///         _ => {log!("Button action not handled");}
///     }
/// }
pub fn build_handle_event_sub_fns(
    fns: &mut Vec<MakepadAction>,
    actions: &Vec<BindAction>,
    binds: Option<&Vec<BindProp>>,
) -> String {
    build_match_handle_event(
        fns,
        actions,
        binds,
        |actions, tag, id, action_code_list| -> String {
            for (action, code) in action_code_list {
                let _ = actions.push(format!(
                    "{} => {{ {} }}",
                    MakepadWidgetActions::match_action(tag.into(), action).to_string(),
                    code
                ));
            }
            actions.push("_ => ()".to_string());

            format!(
            "for action in cx.capture_actions(|cx| self.{}(id!({})).handle_event(cx, event, scope)) {{ match action.as_widget_action().cast(){{ {} }} }}",
            tag, id, actions.join(", ")
        )
        },
    )
}

pub fn build_match_handle_event<F>(
    fns: &mut Vec<MakepadAction>,
    actions: &Vec<BindAction>,
    binds: Option<&Vec<BindProp>>,
    f: F,
) -> String
where
    F: Fn(&mut Vec<String>, &str, &str, Vec<(&String, String)>) -> String,
{
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
                    .push((action, f.to_code(binds)));
            }
            None => {}
        }
    }
    action_fn
        .into_iter()
        .map(|((tag, id), v)| {
            let tag = camel_to_snake(tag);
            let mut action_str = Vec::new();
            // for (action, code) in v {
            //     action_str.push(f(&tag,id,&action,&code));
            // }
            f(&mut action_str, &tag, id, v)
        })
        .collect::<String>()
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
pub fn vars_to_string(
    name: String,
    vars: Vec<&NodeVariable>,
    binds: &Vec<BindProp>,
) -> (String, String) {
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

    (instance, build_handle_startup(&mut_setup, &immut_setup))
}

pub fn build_draw_walk_sub_binds(vars: Vec<&NodeVariable>, binds: &Vec<BindProp>) -> String {
    let mut fields = Vec::new();
    for (tag, id, (prop, value)) in binds {
        match vars
            .iter()
            .find(|var| var.get_name() == value.get_bind_key())
        {
            Some(var) => {
                // let init = var.init_to_string().unwrap();

                if tag != "Component" {
                    let r = PropRole::try_from((tag, (prop, *var))).unwrap();
                    fields.push((r, tag, id));
                }
            }
            None => {}
        }
    }

    build_setup(fields, |tag, id, props| {
        format!(
            "let {}_{} = self.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
            tag, id, tag, id, tag, id, props
        )
    })
}

/// build normal is aim to add other immuatable properties into start_up function
/// `(value, tag, id)`
pub fn build_normal(fields: Vec<(PropRole, &String, &String)>) -> String {
    build_setup(fields, |tag, id, props| {
        format!(
            "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
            tag, id, tag, id, tag, id, props
        )
    })
}

fn build_setup<F>(fields: Vec<(PropRole, &String, &String)>, f: F) -> String
where
    F: Fn(&str, &str, &str) -> String,
{
    let mut setup = HashMap::new();
    for (prop, tag, id) in fields {
        let tag = camel_to_snake(tag);
        setup.entry((tag, id)).or_insert_with(Vec::new).push(prop);
    }

    setup
        .into_iter()
        .map(|((tag, id), v)| {
            let widget_name = snake_to_camel(&tag);

            let props = props_to_string(&widget_name, &v);
            // format!(
            //     "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
            //     tag, id, tag, id, tag, id, props
            // )
            f(&tag, id, &props)
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
            let widget_name = snake_to_camel(&tag);

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

pub fn get_component_prop_struct_name(
    binds: Option<&Vec<BindProp>>,
    vars: &Vec<&NodeVariable>,
) -> Option<String> {
    let binds = binds?;

    // Use iterator chaining to simplify finding the required prop and struct name
    binds.iter()
         .find(|bind| bind.2 .0 == "$props")
         .and_then(|(_, _, (_, prop))| {
             let prop_name = prop.get_bind_key();
             vars.iter()
                 .find(|var| var.get_name() == prop_name)
                 .and_then(|var| {
                     var.get_init().and_then(|init| {
                         if let Expr::Call(expr_call) = &*init.expr {
                             if let Expr::Path(expr_path) = &*expr_call.func {
                                 // Directly return the struct name if found
                                 return Some(expr_path.path.segments[0].ident.to_string());
                             }
                         }
                         // If cannot destructure the expression to find the struct name, panic
                         panic!("can not deep expr struct");
                     })
                 })
                 // If a struct corresponding to the prop_name is not found, panic
                 .or_else(|| panic!("can not find prop struct, you must define prop struct if you use component `:props`"))
         })
}

/// Convert std struct to Makepad Struct
/// ```
/// #[derive(Default)]
/// pub struct MyProps{
///     pub label1: String
/// }
/// //to makepad---------------------------------------
/// #[derive(Default, Live, LiveHook, LiveRegister)]
/// #[live_ignore]
/// pub struct MyProps {
///     #[live]
///     pub label1: RcStringMut,
/// }
/// ```
pub fn build_component_structs(
    mut structs: Vec<ItemStruct>,
    target: Option<String>,
    root: &str,
    inherits: &Widgets,
) -> String {
    let mut prop_name = String::new();
    let mut f = structs
        .iter_mut()
        .map(|item| {
            let name = item.ident.to_string();
            if target.is_some() && name.eq(target.as_ref().unwrap()) {
                prop_name = name;
                // 添加新的派生和属性
                let derives = vec!["Live", "LiveHook", "LiveRegister"];
                item.attrs.iter_mut().for_each(|attr| match &mut attr.meta {
                    Meta::List(d_macro) => {
                        derives.iter().for_each(|item| {
                            d_macro
                                .tokens
                                .append(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                            d_macro
                                .tokens
                                .append(TokenTree::Ident(Ident::new(item, Span::call_site())));
                        });
                    }
                    _ => panic!("Rule Fatal: UnAcceptable! `build_component_structs()`"),
                });
                // add live_ignore attr macro
                let attr_macro = build_attr_macro("live_ignore");
                item.attrs.push(attr_macro);
                // handle field to Makepad struct
                item.fields.iter_mut().for_each(|field| {
                    let _ = MakepadFieldConverter::convert(field);
                });
            }

            item.to_token_stream().to_string()
        })
        .collect::<String>();

    f.push_str(
        format!(
            "#[derive(Live, LiveHook, Widget)] pub struct {} {} #[deref] #[redraw] instance: {},",
            root,
            LEFT_HOLDER,
            inherits.to_string(),
        )
        .as_str(),
    );

    if !prop_name.is_empty() {
        f.push_str(format!("#[live] props: {}", prop_name).as_str());
    }

    f.push_str(RIGHT_HOLDER);
    f
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
