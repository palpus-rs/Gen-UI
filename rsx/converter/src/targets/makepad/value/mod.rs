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

pub use align::{Align, DAlign};
pub use color::Color;
pub use cursor::Cursor;
pub use event::EventOrder;
pub use flow::Flow;
pub use margin::Margin;
pub use optimize::{Optimize, ViewOptimize};
pub use padding::Padding;
pub use size::Size;
pub use vecs::DVec2;

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum MakepadPropValue {
    String(String),
    F64(f64),
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
    Bind(String, Option<Box<MakepadPropValue>>),
}

impl MakepadPropValue {
    pub fn bind(k: &str, v: Option<Box<MakepadPropValue>>) -> Self {
        MakepadPropValue::Bind(k.to_string(), v)
    }
    pub fn bind_without_value(k: &str) -> Self {
        Self::bind(k, None)
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
        }
        panic!("not a bind MakepadPropValue")
    }
    // pub fn rs_to_mkpad_value(ty: &str, input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
    //     let value = match ty {

    //     };
    //     Ok(value)
    // }
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
            MakepadPropValue::String(s) => f.write_str(s),
            MakepadPropValue::Size(s) => f.write_str(s.to_string().as_str()),
            MakepadPropValue::Color(c) => {
                f.write_fmt(format_args!("{{ color: {} }}", c.to_string()))
            }
            MakepadPropValue::Bool(b) => f.write_str(&b.to_string()),
            MakepadPropValue::Margin(m) => f.write_str(m.to_string().as_str()),
            MakepadPropValue::Padding(p) => f.write_str(p.to_string().as_str()),
            MakepadPropValue::F64(num) => f.write_str(num.to_string().as_str()),
            MakepadPropValue::Align(a) => f.write_str(a.to_string().as_str()),
            MakepadPropValue::Flow(flow) => f.write_str(flow.to_string().as_str()),
            MakepadPropValue::DVec2(dv) => f.write_str(dv.to_string().as_str()),
            MakepadPropValue::Optimize(o) => f.write_str(o.to_string().as_str()),
            MakepadPropValue::EventOrder(eo) => f.write_str(eo.to_string().as_str()),
            MakepadPropValue::Cursor(c) => f.write_str(c.to_string().as_str()),
            MakepadPropValue::Bind(_k, v) => f.write_str(v.clone().unwrap().to_string().as_str()),
        }
    }
}
