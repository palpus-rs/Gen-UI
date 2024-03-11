use std::fmt::Display;

use syn::{LocalInit, Type};

#[derive(Debug,Clone,PartialEq)]
pub struct NodeVariable<'a> {
    name: String,
    ty: String,
    init: Option<&'a LocalInit>,
}

impl<'a> NodeVariable<'a> {
    pub fn new(name: &'a str, ty:&'a str, init: Option<&'a LocalInit>) -> Self {
        Self::new_unwrap(name.to_string(), ty.to_string(),init)
    }
    pub fn new_unwrap(name: String, ty:String,init: Option<&'a LocalInit>) -> Self {
        NodeVariable{
            name,
            ty,
            init,
        }
    }
}

impl<'a> Display for NodeVariable<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#[rust] {}: {}", self.name, self.ty))
    }
}
