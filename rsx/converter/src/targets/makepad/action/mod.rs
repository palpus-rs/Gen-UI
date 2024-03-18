mod clicked;

pub use clicked::*;
use parser::{common::parse_string, Value};
use quote::{format_ident, quote};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use proc_macro2::TokenStream;
use syn::{parse_str, visit_mut::VisitMut, Expr, ExprField, ExprPath, Pat};

use crate::utils::alphabetic::camel_to_snake;

use super::{build_normal, prop, BindProp, PropRole};

#[derive(Debug, Clone, PartialEq)]
pub struct MakepadAction {
    name: String,
    value: Expr,
    is_mut: bool,
}

impl MakepadAction {
    pub fn new(name: &str, value: Expr, is_mut: bool) -> Self {
        MakepadAction {
            name: name.to_string(),
            value,
            is_mut,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_value(&self) -> &Expr {
        &self.value
    }
    // pub fn get_args(&self) -> String {
    //     if let Expr::Closure(c) = self.value {
    //         let attrs = c.attrs;
    //         quote! {#attrs}.to_string()
    //     } else {
    //         todo!("wait for impl fn")
    //     }
    // }

    pub fn to_code_string(&mut self, binds: Option<&Vec<BindProp>>) -> (String, Option<String>) {
        // is closure
        match binds {
            Some(binds) => {
                let targets = binds
                    .into_iter()
                    .map(|v| v.clone())
                    .collect::<Vec<BindProp>>();
                let mut replacer = IdentReplacer::new(targets);
                if let Expr::Closure(c) = &mut self.value {
                    let _ = replacer.visit_expr_closure_mut(c);
                    // remove duplicate
                    for item in replacer.props.values_mut() {
                        item.sort();
                        item.dedup();
                    }
                    let mut redraw_prop = replacer.props;
                    let redraw_str = redraw_prop.into_iter().map(|((tag_name, id),prop_kvs)| {

                        let prop_str = prop_kvs.into_iter().map(|(k,v)| format!("{}: (self.instance.get_{}()),",k,v)).collect::<String>();

                        format!( "let {}_{} = self.ui.{}(id!({})); {}_{}.apply_over_and_redraw(cx, live!{{ {} }});",
                        tag_name, id, tag_name, id, tag_name,id, prop_str
                     )
                    }).collect::<String>();

                    return (quote! {#c}.to_string(), Some(redraw_str));
                } else {
                    todo!("wait for impl fn")
                };
            }
            None => {
                let value = self.get_value();
                (quote! {#value}.to_string(), None)
            }
        }
    }
    pub fn to_code(&mut self, binds: Option<&Vec<BindProp>>) -> String {
        let mut res = String::from("let");
        let name = self.name.to_string();
        if self.is_mut {
            res.push_str(" mut");
        }

        let (action_code, redaw_code) = self.to_code_string(binds);

        res.push_str(&format!(" {} = {}; {}();", &name, action_code, &name));

        if redaw_code.is_some() {
            res.push_str(&redaw_code.unwrap());
        }

        res
    }
}

// impl Display for MakepadAction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
// }

/// use this visitor to replace ident
struct IdentReplacer {
    target: Vec<BindProp>,
    /// this props means which bind props are being replaced and replaced value
    props: HashMap<(String, String), Vec<(String, String)>>,
}

impl IdentReplacer {
    pub fn new(target: Vec<BindProp>) -> IdentReplacer {
        IdentReplacer {
            target,
            props: HashMap::new(),
        }
    }
    pub fn prop_names(&self) -> Vec<String> {
        self.target
            .iter()
            .map(|(_, _, (prop_name, _))| prop_name.to_string())
            .collect()
    }
    pub fn insert(&mut self, tag_name: &str, id: &str, prop_name: &str, prop_var_name: &str) -> () {
        let _ = self
            .props
            .entry((camel_to_snake(tag_name).to_string(), id.to_string()))
            .or_default()
            .push((prop_name.to_string(), prop_var_name.to_string()));
    }
}

impl VisitMut for IdentReplacer {
    // fn visit_ident_mut(&mut self, i: &mut proc_macro2::Ident) {
    //     let ident = i.to_string();
    //     if self.target.contains(&ident) {
    //         let new_ident = format_ident!("self.ui.instance.{}", ident);
    //         *i = new_ident;
    //     }
    // }
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        match i {
            Expr::Path(ExprPath { path, .. }) => {
                if let Some(ident) = path.get_ident() {
                    // if self.prop_names().contains(&ident.to_string()) {
                    //     let nex_expr =
                    //         syn::parse_str(&format!("self.ui.instance.{}", ident.to_string()))
                    //             .unwrap();
                    //     *i = nex_expr;
                    // }

                    if let Some((tag_name, id, (prop_name, prop_value))) =
                        self.target.clone().iter().find(|(_, _, (_, prop_value))| {
                            ident.to_string().eq(prop_value.get_bind_key())
                        })
                    {
                        let new_expr =
                            syn::parse_str(&format!("self.instance.{}", prop_value.get_bind_key()))
                                .unwrap();
                        *i = new_expr;

                        self.insert(tag_name, id, prop_name, prop_value.get_bind_key());
                    }
                }
            }
            _ => syn::visit_mut::visit_expr_mut(self, i),
        }
    }

    fn visit_expr_closure_mut(&mut self, i: &mut syn::ExprClosure) {
        syn::visit_mut::visit_expr_closure_mut(self, i);
    }
}
