mod clicked;

pub use clicked::*;
use std::fmt::Display;

use proc_macro2::TokenStream;
use syn::{Expr, ExprClosure};

use super::BindProp;

#[derive(Debug, Clone, PartialEq)]
pub struct MakepadAction {
    name: String,
    value: Expr,
}

impl MakepadAction {
    pub fn new(name: &str, value: Expr) -> Self {
        MakepadAction {
            name: name.to_string(),
            value,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_value(&self) -> &Expr {
        &self.value
    }
    pub fn to_code(&self) -> TokenStream {
        todo!()
    }
    pub fn to_code_string(&self, binds: Option<&Vec<BindProp>>) -> String {
        // is closure
        if let Expr::Closure(ExprClosure { body, .. }) = &self.value {
            // change closure inner variable if is bind or not change
            let body = *body.clone();
            dbg!(&body);
            if let Expr::Block(block) = body {
                dbg!(block);
                todo!()
                // let stmts = &mut block.block.stmts;
                // for stmt in stmts {
                //     if let syn::Stmt::Local(local) = stmt {
                //         if let Some(binds) = binds {
                //             for (_, _, (prop_name, _)) in binds {
                //                 if let syn::Pat::Ident(pat) = &*local.pat {
                //                     if pat.ident == prop_name {
                //                         let _ = local.init.replace(bind.get_value().clone());
                //                     }
                //                 }
                //             }
                //         }
                //     }
                // }
            }
        }
        panic!("Not implemented")
    }
}

// impl Display for MakepadAction {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
// }
