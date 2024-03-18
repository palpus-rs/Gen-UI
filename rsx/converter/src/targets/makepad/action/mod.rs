mod clicked;

pub use clicked::*;
use parser::{common::parse_string, Value};
use quote::{format_ident, quote};
use std::fmt::Display;

use proc_macro2::TokenStream;
use syn::{parse_str, visit_mut::VisitMut, Expr, ExprField, ExprPath, Pat};

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

    pub fn to_code_string(&mut self, binds: Option<&Vec<BindProp>>) -> String {
        // is closure
        match binds {
            Some(binds) => {
                let targets = binds
                    .into_iter()
                    .map(|v| v.clone())
                    .collect::<Vec<BindProp>>();
                let mut replacer = IdentReplacer::new(targets);
                if let Expr::Closure(c) = &mut self.value {
                    dbg!(&c);
                    let _ = replacer.visit_expr_closure_mut(c);
                    return quote! {#c}.to_string();
                } else {
                    todo!("wait for impl fn")
                };
            }
            None => {
                let value = self.get_value();
                quote! {#value}.to_string()
            }
        }
    }
    pub fn to_code(&mut self, binds: Option<&Vec<BindProp>>) -> String {
        let mut res = String::from("let");
        let name = self.name.to_string();
        if self.is_mut {
            res.push_str(" mut");
        }

        // let p = build_normal(fields)

        res.push_str(&format!(
            " {} = {}; {}();",
            &name,
            self.to_code_string(binds),
            &name
        ));

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
    props: Vec<(PropRole, String, String)>,
}

impl IdentReplacer {
    pub fn new(target: Vec<BindProp>) -> IdentReplacer {
        IdentReplacer {
            target,
            props: Vec::new(),
        }
    }
    pub fn prop_names(&self) -> Vec<String> {
        self.target
            .iter()
            .map(|(_, _, (prop_name, _))| prop_name.to_string())
            .collect()
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
                        self.target.iter().find(|(_, _, (_, prop_value))| {
                            ident.to_string().eq(prop_value.get_bind_key())
                        })
                    {
                        let nex_expr = syn::parse_str(&format!(
                            "self.ui.instance.{}",
                            prop_value.get_bind_key()
                        ))
                        .unwrap();
                        *i = nex_expr;
                        // self.props.push(PropRole::try_from((tag_name,(prop_name,Value::UnKnown()))))
                    }
                }
            }
            _ => syn::visit_mut::visit_expr_mut(self, i),
        }
    }

    fn visit_expr_closure_mut(&mut self, i: &mut syn::ExprClosure) {
        syn::visit_mut::visit_expr_closure_mut(self, i);
    }
    // fn visit_expr_assign_mut(&mut self, i: &mut syn::ExprAssign) {
    //     //get right when assign
    //     let right = *i.right;
    // }

    // fn visit_block_mut(&mut self, i: &mut syn::Block) {
    //     i.stmts.iter_mut().for_each(|stmt|{

    //     })
    // }
}
