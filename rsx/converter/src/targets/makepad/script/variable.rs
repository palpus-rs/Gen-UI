use std::fmt::Display;

use quote::quote;
use syn::{LocalInit, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeVariable {
    name: String,
    ty: String,
    init: Option<LocalInit>,
}

impl NodeVariable {
    pub fn new(name: &str, ty: &str, init: Option<LocalInit>) -> Self {
        Self::new_unwrap(name.to_string(), ty.to_string(), init)
    }
    pub fn new_unwrap(name: String, ty: String, init: Option<LocalInit>) -> Self {
        NodeVariable { name, ty, init }
    }
    pub fn init_to_string(&self) -> Option<String> {
        match &self.init {
            Some(init) => {
                let expr = &*init.expr;
                let expr_token = quote! {#expr}.to_string();
                Some(expr_token)
            }
            None => None,
        }
    }
}

impl Display for NodeVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#[rust] {}: {}", self.name, self.ty))
    }
}
