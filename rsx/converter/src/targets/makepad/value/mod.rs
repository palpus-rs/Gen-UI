mod align;
mod color;
mod cursor;
mod event;
mod flow;
mod margin;
mod optimize;
mod padding;
mod size;
mod vecs;
mod wrap;

pub use align::{Align, DAlign};
pub use color::Color;
pub use cursor::Cursor;
pub use event::EventOrder;
pub use flow::Flow;
pub use margin::Margin;
#[allow(unused_imports)]
pub use optimize::{Optimize, ViewOptimize};
pub use padding::Padding;
use quote::quote;
pub use size::Size;
use syn::Expr;
pub use vecs::DVec2;
pub use wrap::TextWrap;

use super::PropRole;
use std::fmt::Display;

pub trait MapValue {
    /// map struct to makepad value code
    fn map_value_code(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum MakepadPropValue {
    String(String),
    StrVec(String),
    F64(f64),
    F32(f32),
    Size(Size),
    Color(Color),
    Bool(bool),
    Margin(Margin),
    Padding(Padding),
    Align(Align),
    Flow(Flow),
    DVec2(DVec2),
    Optimize(Optimize),
    EventOrder(EventOrder),
    Cursor(Cursor),
    TextWrap(TextWrap),
    Font(String),
    Bind(String, Option<Box<MakepadPropValue>>),
    Function(String, Option<Expr>),
}

impl MakepadPropValue {
    pub fn bind(k: &str, v: Option<Box<MakepadPropValue>>) -> Self {
        MakepadPropValue::Bind(k.to_string(), v)
    }
    pub fn func(k: &str, v: Option<Expr>) -> Self {
        MakepadPropValue::Function(k.to_string(), v)
    }
    pub fn bind_without_value(k: &str) -> Self {
        Self::bind(k, None)
    }
    pub fn get_fn_key(&self) -> &str {
        if let MakepadPropValue::Function(k, _) = self {
            return k;
        }
        panic!("not a function MakepadPropValue")
    }
    pub fn fn_without_value(k: &str) -> Self {
        Self::func(k, None)
    }
    pub fn get_bind_key(&self) -> &str {
        if let MakepadPropValue::Bind(k, _) = self {
            return k;
        }
        panic!("not a bind MakepadPropValue")
    }
    pub fn set_bind_value(&mut self, v: MakepadPropValue) -> () {
        if let MakepadPropValue::Bind(_, ref mut value) = self {
            let _ = value.replace(Box::new(v));
        } else {
            panic!("not a bind MakepadPropValue");
        }
    }
    pub fn to_makepad_ty(&self) -> String {
        match self {
            MakepadPropValue::String(_) => "String".to_string(),
            MakepadPropValue::StrVec(_) => "String".to_string(),
            MakepadPropValue::F64(_) => "f64".to_string(),
            MakepadPropValue::F32(_) => "f32".to_string(),
            MakepadPropValue::Size(_) => "Size".to_string(),
            MakepadPropValue::Color(_) => "String".to_string(),
            MakepadPropValue::Bool(_) => "bool".to_string(),
            MakepadPropValue::Margin(_) => "Margin".to_string(),
            MakepadPropValue::Padding(_) => "Padding".to_string(),
            MakepadPropValue::Align(_) => "Align".to_string(),
            MakepadPropValue::Flow(_) => "Flow".to_string(),
            MakepadPropValue::DVec2(_) => "DVec2".to_string(),
            MakepadPropValue::Optimize(_) => "Optimize".to_string(),
            MakepadPropValue::EventOrder(_) => "EventOrder".to_string(),
            MakepadPropValue::Cursor(_) => "MouseCursor".to_string(),
            MakepadPropValue::TextWrap(_) => "TextWrap".to_string(),
            MakepadPropValue::Font(_) => panic!("v1.0 Font can not be bind"),
            MakepadPropValue::Bind(_, v) => {
                if let Some(v) = v {
                    v.to_makepad_ty()
                } else {
                    panic!("bind value is none")
                }
            }
            MakepadPropValue::Function(_, _) => todo!(),
        }
    }
    pub fn to_value_code(&self) -> String {
        match self {
            MakepadPropValue::String(s) => format!("String::from(\"{}\")", s),
            MakepadPropValue::StrVec(s) => format!("String::from(\"{}\")", s),
            MakepadPropValue::F64(f) => f.to_string(),
            MakepadPropValue::F32(f) => f.to_string(),
            MakepadPropValue::Size(s) => s.map_value_code(),
            MakepadPropValue::Color(c) => c.map_value_code(),
            MakepadPropValue::Bool(b) => b.to_string(),
            MakepadPropValue::Margin(m) => m.map_value_code(),
            MakepadPropValue::Padding(p) => p.map_value_code(),
            MakepadPropValue::Align(a) => a.map_value_code(),
            MakepadPropValue::Flow(f) => f.map_value_code(),
            MakepadPropValue::DVec2(v) => v.map_value_code(),
            MakepadPropValue::Optimize(o) => o.map_value_code(),
            MakepadPropValue::EventOrder(eo) => eo.map_value_code(),
            MakepadPropValue::Cursor(c) => c.map_value_code(),
            MakepadPropValue::TextWrap(tw) => tw.map_value_code(),
            MakepadPropValue::Font(_) => panic!("v1.0 Font can not be bind"),
            MakepadPropValue::Bind(_, v) => {
                if let Some(v) = v {
                    v.to_value_code()
                } else {
                    panic!("bind value is none")
                }
            }
            MakepadPropValue::Function(_, v) => {
                if let Some(v) = v {
                    quote! {#v}.to_string()
                } else {
                    panic!("function value is none")
                }
            }
        }
    }
}

// impl Parse for MakepadPropValue {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let ident = input.parse::<syn::Ident>()?;
//         dbg!(&ident.to_string());
//         let _ = input.parse::<syn::Token![:]>();

//         Ok(value)
//     }
// }

impl Display for MakepadPropValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MakepadPropValue::String(s) => f.write_fmt(format_args!("\"{}\"", s)),
            MakepadPropValue::StrVec(s) => f.write_str(s),
            MakepadPropValue::Size(s) => f.write_str(s.to_string().as_str()),
            MakepadPropValue::Color(c) => {
                if c.is_font() {
                    f.write_str(c.to_string().as_str())
                } else {
                    f.write_fmt(format_args!("{{ color: {} }}", c.to_string()))
                }
            }
            MakepadPropValue::Bool(b) => f.write_str(&b.to_string()),
            MakepadPropValue::Margin(m) => f.write_str(m.to_string().as_str()),
            MakepadPropValue::Padding(p) => f.write_str(p.to_string().as_str()),
            MakepadPropValue::F64(num) => f.write_str(num.to_string().as_str()),
            MakepadPropValue::F32(num) => f.write_str(num.to_string().as_str()),
            MakepadPropValue::Align(a) => f.write_str(a.to_string().as_str()),
            MakepadPropValue::Flow(flow) => f.write_str(flow.to_string().as_str()),
            MakepadPropValue::DVec2(dv) => f.write_str(dv.to_string().as_str()),
            MakepadPropValue::Optimize(o) => f.write_str(o.to_string().as_str()),
            MakepadPropValue::EventOrder(eo) => f.write_str(eo.to_string().as_str()),
            MakepadPropValue::Cursor(c) => f.write_str(c.to_string().as_str()),
            MakepadPropValue::Bind(_k, v) => f.write_str(v.clone().unwrap().to_string().as_str()),
            MakepadPropValue::TextWrap(tw) => f.write_str(tw.to_string().as_str()),
            MakepadPropValue::Font(font) => f.write_fmt(format_args!("{{path: dep({})}}", font)),
            MakepadPropValue::Function(_, _) => todo!(),
        }
    }
}

impl From<PropRole> for MakepadPropValue {
    fn from(value: PropRole) -> Self {
        match value {
            PropRole::Normal(_, v) => v,
            PropRole::Bind(_, v) => v,
            PropRole::Function(_, func) => func,
            PropRole::Context(_) => todo!(),
            PropRole::Special(_) => todo!(),
        }
    }
}
