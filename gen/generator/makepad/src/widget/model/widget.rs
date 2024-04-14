use std::collections::HashMap;

use crate::widget::BuiltIn;

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
    pub props: Option<HashMap<String, String>>,
    pub children: Option<Vec<Widget>>,
    pub inherits: BuiltIn,
    
}
