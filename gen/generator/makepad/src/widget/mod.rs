//! 将GenUI的Model转换为Makepad的Model，分为两种形式
//! - 1. AppMain：表示整个应用的入口
//! - 2. Widget：表示一个组件
//! 这两种形式都会包含两个部分：
//! - live_design! 宏编写的DSL模板部分（必须有）
//! - 构建这个模板的代码部分（可能有）
//!  
use core::panic;
use std::fmt::Display;
#[allow(unused_imports)]
use std::{collections::HashMap, default, fmt::Debug};

use gen_converter::model::script::PropFn;
use gen_parser::{PropsKey, Value};
use gen_utils::{common::snake_to_camel, error::Errors};
use proc_macro2::TokenStream;
use syn::{Ident, ItemStruct};

use crate::{str_to_string_try_from, ToToken};

pub mod area;
pub mod button;
pub mod checkbox;
pub mod color_picker;
pub mod define;
pub mod html;
pub mod icon;
pub mod image;
pub mod label;
pub mod markdown;
pub mod model;
pub mod radio;
pub mod root;
pub mod scroll;
pub mod splitter;
pub mod text_input;
pub mod utils;
pub mod view;
pub mod window;
pub mod window_menu;
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
const ICON: &str = "Icon";
const IMAGE: &str = "Image";
const RADIO: &str = "RadioButton";
const CHECKBOX: &str = "CheckBox";
const TEXT_INPUT: &str = "TextInput";
const ROOT: &str = "Root";
const SCROLLXVIEW: &str = "ScrollXView";
const SCROLLYVIEW: &str = "ScrollYView";
const SCROLLXYVIEW: &str = "ScrollXYView";

pub fn prop_ignore(prop: &str) -> bool {
    ["id", "class"].contains(&prop)
}

#[derive(Debug, Clone, Default)]
pub enum BuiltIn {
    Window,
    View,
    ScrollXView,
    ScrollYView,
    ScrollXYView,
    TextInput,
    Label,
    Button,
    #[default]
    Area,
    Icon,
    Image,
    CheckBox,
    Radio,
    Root,
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
            BuiltIn::Area => todo!("area do not need to bind prop"),
            BuiltIn::Icon => icon::IconProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Image => image::ImageProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::CheckBox => checkbox::CheckBoxProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Radio => radio::RadioButtonProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::Root => root::RootProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollXView => view::ScrollXViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollYView => view::ScrollYViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::ScrollXYView => view::ScrollXYViewProps::prop_bind(prop, value, is_prop, ident),
            BuiltIn::TextInput => text_input::TextInputProps::prop_bind(prop, value, is_prop, ident),
        }
    }
    /// 对内置组件的属性进行处理
    pub fn props(&self, props: &HashMap<PropsKey, Value>) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowProps::props(props).to_token_stream(),
            BuiltIn::View => view::ViewProps::props(props).to_token_stream(),
            BuiltIn::Label => label::LabelProps::props(props).to_token_stream(),
            BuiltIn::Button => button::ButtonProps::props(props).to_token_stream(),
            BuiltIn::Icon => icon::IconProps::props(props).to_token_stream(),
            BuiltIn::Image => image::ImageProps::props(props).to_token_stream(),
            BuiltIn::CheckBox => checkbox::CheckBoxProps::props(props).to_token_stream(),
            BuiltIn::Radio => radio::RadioButtonProps::props(props).to_token_stream(),
            BuiltIn::Root => root::RootProps::props(props).to_token_stream(),
            BuiltIn::ScrollXView => view::ScrollXViewProps::props(props).to_token_stream(),
            BuiltIn::ScrollYView => view::ScrollYViewProps::props(props).to_token_stream(),
            BuiltIn::ScrollXYView => view::ScrollXYViewProps::props(props).to_token_stream(),
            BuiltIn::Area => todo!("area do not need to bind static prop"),
            BuiltIn::TextInput => text_input::TextInputProps::props(props).to_token_stream(),
        }
    }
    pub fn to_token_stream(&self, ptr: &ItemStruct) -> TokenStream {
        match self {
            BuiltIn::Window => window::WindowPropPtr::from(ptr).to_token_stream(),
            BuiltIn::View => view::ViewPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Label => label::LabelPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Button => button::ButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Area => area::AreaPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Icon => icon::IconPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Image => image::ImagePropPtr::from(ptr).to_token_stream(),
            BuiltIn::CheckBox => checkbox::CheckBoxPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Radio => radio::RadioButtonPropPtr::from(ptr).to_token_stream(),
            BuiltIn::Root => root::RootPropPtr::from(ptr).to_token_stream(),
            BuiltIn::ScrollXView | BuiltIn::ScrollYView | BuiltIn::ScrollXYView => {
                panic!("scroll view can not be inherited you need to inherits View")
            },
            BuiltIn::TextInput => text_input::TextInputPropPtr::from(ptr).to_token_stream(),
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
    pub fn draw_walk(&self, draw_walk: &Option<TokenStream>) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::draw_walk(),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::draw_walk(draw_walk),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => root::draw_walk(),
            BuiltIn::ScrollXView | BuiltIn::ScrollYView | BuiltIn::ScrollXYView => {
                panic!("scroll view can not be inherited, so that it can not draw_walk, you need to inherits View")
            },
            BuiltIn::TextInput => todo!(),
        }
    }
    /// 处理widget的事件处理函数
    pub fn handle_event(
        &self,
        event: &Option<Vec<PropFn>>,
        props: &Option<Vec<PropFn>>,
        instance_name: Option<&Ident>,
        prop_fields: Option<&Vec<Ident>>,
    ) -> TokenStream {
        match self {
            BuiltIn::Window => todo!(),
            BuiltIn::View => view::handle_event(event, props, instance_name, prop_fields),
            BuiltIn::Label => todo!(),
            BuiltIn::Button => todo!(),
            BuiltIn::Area => area::handle_event(event, props, instance_name, prop_fields),
            BuiltIn::Icon => todo!(),
            BuiltIn::Image => todo!(),
            BuiltIn::CheckBox => todo!(),
            BuiltIn::Radio => todo!(),
            BuiltIn::Root => root::handle_event(event, props, instance_name, prop_fields),
            BuiltIn::ScrollXView | BuiltIn::ScrollYView | BuiltIn::ScrollXYView => {
                panic!("scroll view can not be inherited, so that it can not handle_event, you need to inherits View")
            },
            BuiltIn::TextInput => todo!(),
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
            ICON => Ok(BuiltIn::Icon),
            IMAGE => Ok(BuiltIn::Image),
            CHECKBOX => Ok(BuiltIn::CheckBox),
            RADIO => Ok(BuiltIn::Radio),
            ROOT => Ok(BuiltIn::Root),
            SCROLLXVIEW => Ok(BuiltIn::ScrollXView),
            SCROLLYVIEW => Ok(BuiltIn::ScrollYView),
            SCROLLXYVIEW => Ok(BuiltIn::ScrollXYView),
            TEXT_INPUT => Ok(BuiltIn::TextInput),
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

impl Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BuiltIn::Window => WINDOW,
            BuiltIn::View => VIEW,
            BuiltIn::Label => LABEL,
            BuiltIn::Button => BUTTON,
            BuiltIn::Area => AREA,
            BuiltIn::Icon => ICON,
            BuiltIn::Image => IMAGE,
            BuiltIn::CheckBox => CHECKBOX,
            BuiltIn::Radio => RADIO,
            BuiltIn::Root => ROOT,
            BuiltIn::ScrollXView => SCROLLXVIEW,
            BuiltIn::ScrollYView => SCROLLYVIEW,
            BuiltIn::ScrollXYView => SCROLLXYVIEW,
            BuiltIn::TextInput => TEXT_INPUT,
        })
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
