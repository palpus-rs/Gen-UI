use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::widget::BuiltIn;

use super::traits::WidgetTrait;

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
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
    pub inherits: BuiltIn,
    pub traits: WidgetTrait,
}
