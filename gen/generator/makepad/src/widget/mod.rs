//! 将GenUI的Model转换为Makepad的Model，分为两种形式
//! - 1. AppMain：表示整个应用的入口
//! - 2. Widget：表示一个组件
//! 这两种形式都会包含两个部分：
//! - live_design! 宏编写的DSL模板部分（必须有）
//! - 构建这个模板的代码部分（可能有）
//!  
use core::panic;
#[allow(unused_imports)]
use std::{collections::HashMap, default, fmt::Debug};

use gen_converter::{error::Errors, model::script::PropFn};
use gen_parser::{PropsKey, Value};
use gen_utils::common::snake_to_camel;
use proc_macro2::TokenStream;
use syn::ItemStruct;

use crate::{str_to_string_try_from, ToToken};

// use crate::{
//     gen::{FieldItem, FieldTable},
//     utils::{apply_over_and_redraw, struct_field_type},
// };

pub mod area;
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
/// 表示GenUI的声明的单独的一个组件，不是内置组件
/// 但它会直接认为是Makepad的Area
const COMPONENT: &str = "Component";

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
    /// 处理内置组件绑定动态属性
    pub fn prop_bind(
        &self,
        prop: &PropsKey,
        value: &Value,
        is_prop: bool,
        ident: &str,
    ) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::View => view::ViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Label => label::LabelProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Button => button::ButtonProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Area => todo!(),
        }
    }
    /// 对内置组件的属性进行处理
    pub fn props(&self, props: &HashMap<PropsKey, Value>) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::props(props).to_token_stream(),
            BuiltIn::View => view::ViewProps::props(props).to_token_stream(),
            BuiltIn::Label => label::LabelProps::props(props).to_token_stream(),
            BuiltIn::Button => button::ButtonProps::props(props).to_token_stream(),
            _ => panic!("only built-in widget can be get"),
        }
    }
    pub fn to_token_stream(&self, ptr: &ItemStruct) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowPropPtr::from(ptr).to_token_stream(),
            BuiltIn::View => view::ViewPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Label => label::LabelPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Button => button::ButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Area => area::AreaPropPtr::from(ptr).to_token_stream(),
        }
    }
    pub fn has_event(&self) -> bool {
        match self {
            BuiltIn::Button => true,
            _ => false,
        }
    }
    /// you mut be sure that the value is a built-in widget
    pub fn from(value: &str) -> Self {
        value.try_into().unwrap()
    }
    /// 处理widget的draw_walk绘制函数
    pub fn draw_walk(&self, draw_walk: &Option<Vec<PropFn>>) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::draw_walk(draw_walk),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::draw_walk(draw_walk),
        }
    }
    /// 处理widget的事件处理函数
    pub fn handle_event(&self, event: &Option<Vec<PropFn>>) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::handle_event(event),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::handle_event(event),
        }
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
            BUTTON => Ok(BuiltIn::Button),
            AREA => Ok(BuiltIn::Area),
            COMPONENT => Ok(BuiltIn::Area),
            _ => Err(Errors::BuiltInConvertFail),
        }
    }
}

str_to_string_try_from!(BuiltIn);

impl TryFrom<Option<&String>> for BuiltIn {
    type Error = Errors;
    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {
        if let Some(target) = value {
            target.try_into()
        } else {
            Ok(BuiltIn::Area)
        }
    }
}

pub trait StaticProps: Debug + ToToken {
    fn props(props: &HashMap<PropsKey, Value>) -> Self
    where
        Self: Sized;
    fn prop(&mut self, prop_name: &str, value: Value) -> ();
}

pub trait DynProps {
    fn prop_bind(prop: &PropsKey, value: &Value, is_prop: bool, ident: &str) -> TokenStream;
}
