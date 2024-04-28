//! 将GenUI的Model转换为Makepad的Model，分为两种形式
//! - 1. AppMain：表示整个应用的入口
//! - 2. Widget：表示一个组件
//! 这两种形式都会包含两个部分：
//! - live_design! 宏编写的DSL模板部分（必须有）
//! - 构建这个模板的代码部分（可能有）
//!  
use core::panic;
use std::{
    collections::HashMap,
    default,
    fmt::{Debug, Display},
};

use gen_converter::error::Errors;
use gen_parser::{PropsKey, Value};
use gen_utils::common::{
    snake_to_camel, token_stream_to_tree, token_tree_ident, token_tree_punct_alone,
};
use proc_macro2::{TokenStream, TokenTree};

use crate::{str_to_string_try_from, ToToken};

// use crate::{
//     gen::{FieldItem, FieldTable},
//     utils::{apply_over_and_redraw, struct_field_type},
// };

pub mod button;
pub mod define;
pub mod label;
pub mod model;
pub mod utils;
pub mod view;
pub mod window;

// pub use define::*;
// pub use button::*;
// pub use label::*;
// pub use view::*;
// pub use window::*;

const WINDOW: &str = "Window";
const VIEW: &str = "View";
const LABEL: &str = "Label";
const BUTTON: &str = "Button";
const AREA: &str = "Area";

// #[derive(Debug, Clone, PartialEq, Default)]
// pub enum Widget {
//     Window,
//     #[default]
//     View,
//     Label,
//     Button,
//     Define(String),
// }

// impl Widget {
//     pub fn ast(&self) {
//         match self {
//             Widget::Define(name) => todo!(),
//             _ => todo!(),
//         }
//     }
//     pub fn props(&self, props: &HashMap<PropsKey, Value>) -> Vec<TokenTree> {
//         let mut ast = vec![];
//         props.iter().for_each(|(prop, value)| {
//             let prop_name = prop.name();
//             // 非绑定属性， 绑定的直接忽略
//             match value.is_unknown_and_get() {
//                 Some(prop_value) => {
//                     ast.extend(match self {
//                         Widget::Window => window::prop(prop_name, prop_value),
//                         Widget::View => view::prop(prop_name, prop_value),
//                         Widget::Label => todo!(),
//                         Widget::Button => button::prop(prop_name, prop_value),
//                         Widget::Define(_) => todo!(),
//                     });
//                 }
//                 None => (),
//             }
//         });
//         ast
//     }
//     /// return:
//     /// - pub field: Type token,
//     /// - init code token
//     /// - field token
//     pub fn props_from_tk(
//         &self,
//         root: Option<String>,
//         tag: String,
//         id: String,
//         pvs: Vec<(PropsKey, String, TokenStream, bool)>,
//     ) -> (TokenStream, TokenStream, Vec<FieldItem>, Vec<TokenTree>) {
//         let mut prop_fts = TokenStream::new();
//         let mut props = TokenStream::new();
//         let mut codes = TokenStream::new();
//         // let mut fields = TokenStream::new();
//         let mut fields = vec![];

//         pvs.into_iter().for_each(|(k, ident, code, _)| {
//             let (p_tk, ty_tk) = self.prop_from_str(&k, &ident.as_str());
//             props.extend(p_tk);
//             prop_fts.extend(struct_field_type(&ident, ty_tk));
//             codes.extend(code);
//             // fields.extend(vec![token_tree_ident(&ident), token_tree_punct_alone(',')]);
//             fields.push(FieldItem{
//                 source: self.clone(),
//                 prop: k.name().to_string(),
//                 value: ident,
//                 id: id.clone(),
//             })
//         });
//         (
//             prop_fts,
//             codes,
//             fields,
//             apply_over_and_redraw(root, tag, &id, token_stream_to_tree(props)),
//         )
//     }
//     fn prop_from_str(&self, k: &PropsKey, v: &str) -> (Vec<TokenTree>, TokenTree) {
//         let prop_name = k.name();
//         match self {
//             Widget::Window => todo!(),
//             Widget::View => view::prop_token(prop_name, v),
//             Widget::Label => todo!(),
//             Widget::Button => button::prop_token(prop_name, v),
//             Widget::Define(_) => todo!(),
//         }
//     }
//     pub fn events(
//         &self,
//         root: Option<String>,
//         id: String,
//         pv: (PropsKey, String, TokenStream),
//         field_table: &FieldTable,
//     ) -> Vec<TokenTree> {
//         match self {
//             Widget::Window => todo!(),
//             Widget::View => todo!(),
//             Widget::Label => todo!(),
//             Widget::Button => button::event(root, id, pv, field_table),
//             Widget::Define(_) => todo!(),
//         }
//     }
// }

// impl From<&str> for Widget {
//     fn from(value: &str) -> Self {
//         let widget_name = snake_to_camel(value).unwrap();
//         match widget_name.as_str() {
//             WINDOW => Widget::Window,
//             VIEW => Widget::View,
//             LABEL => Widget::Label,
//             BUTTON => Widget::Button,
//             _ => Widget::Define(widget_name),
//         }
//     }
// }

// impl Display for Widget {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(match self {
//             Widget::Window => WINDOW,
//             Widget::View => VIEW,
//             Widget::Label => LABEL,
//             Widget::Button => BUTTON,
//             Widget::Define(d) => d,
//         })
//     }
// }

pub fn prop_ignore(prop: &str) -> bool {
    ["id", "class"].contains(&prop)
}

#[derive(Debug, Clone, Default)]
pub enum BuiltIn {
    Window,
    View,
    Label,
    Button,
    #[default]
    Area,
}

impl BuiltIn {
    /// 对内置组件的属性进行处理
    pub fn props(&self, props: &HashMap<PropsKey, Value>) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::props(props).to_token_stream(),
            BuiltIn::View =>view::ViewProps::props(props).to_token_stream(),
            BuiltIn::Label => label::LabelProps::props(props).to_token_stream(),
            BuiltIn::Button => button::ButtonProps::props(props).to_token_stream(),
            _ => panic!("only built-in widget can be get"),
        }
    }
    /// you mut be sure that the value is a built-in widget
    pub fn from(value:&str) -> Self{
        value.try_into().unwrap()
    }
}

impl TryFrom<&str> for BuiltIn {
    type Error = Errors;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let widget_name = snake_to_camel(value).unwrap();
        match widget_name.as_str() {
            WINDOW => Ok(BuiltIn::Window),
            VIEW => Ok(BuiltIn::View),
            LABEL => Ok(BuiltIn::Label),
            BUTTON =>Ok( BuiltIn::Button),
            AREA => Ok(BuiltIn::Area),
            _ => Err(Errors::BuiltInConvertFail),
        }
    }
}

str_to_string_try_from!(BuiltIn);

pub trait StaticProps: Debug + ToToken {
    fn props(props: &HashMap<PropsKey, Value>) -> Self
    where
        Self: Sized;
    fn prop(&mut self, prop_name: &str, value: Value) -> ();
}
