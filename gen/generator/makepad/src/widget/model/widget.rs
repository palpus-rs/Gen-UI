use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::widget::{self, BuiltIn};

use super::{live_design::LiveDesign, role::Role, traits::WidgetTrait};

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
    pub live_design: LiveDesign,
    pub is_root: bool,
    pub is_prop: bool,
    pub in_live_design: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    pub name: String,
    /// static_props is props in live_design
    pub static_props: Option<HashMap<String, String>>,
    pub props: Option<HashMap<String, String>>,
    pub events: Option<TokenStream>,
    pub children: Option<Vec<Widget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: WidgetTrait,
    pub role: Role
}

impl Widget {
    pub fn new(name:&str)->Self{
        let mut widget = Widget::default();
        widget.name = name.to_string();
        widget
    }
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(id.to_string());
    }
}

