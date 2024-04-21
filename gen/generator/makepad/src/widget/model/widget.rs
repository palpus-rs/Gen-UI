use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::widget::{self, BuiltIn};

use super::{live_design::LiveDesign, role::Role, traits::WidgetTrait};

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
    /// Makepad live_design! macro
    pub live_design: LiveDesign,
    pub is_root: bool,
    pub is_prop: bool,
    pub in_live_design: bool,
    pub is_built_in: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    pub name: String,
    /// props in live_design
    pub props: Option<HashMap<String, TokenStream>>,
    /// events called in makepad
    pub events: Option<HashMap<String, TokenStream>>,
    pub prop_ptr: Option<TokenStream>,
    pub event_ptr: Option<TokenStream>,
    pub children: Option<Vec<Widget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: WidgetTrait,
    pub role: Role,
}

impl Widget {
    pub fn new(name: &str) -> Self {
        let mut widget = Widget::default();
        widget.name = name.to_string();
        widget
    }
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn set_is_root(&mut self, is_root: bool) -> &mut Self {
        self.is_root = is_root;
        self
    }
    pub fn set_is_prop(&mut self, is_prop: bool) -> &mut Self {
        self.is_prop = is_prop;
        self
    }
    pub fn set_in_live_design(&mut self, in_live_design: bool) -> &mut Self {
        self.in_live_design = in_live_design;
        self
    }
    pub fn set_is_built_in(&mut self, is_built_in: bool) -> &mut Self {
        self.is_built_in = is_built_in;
        self
    }
    pub fn set_props(&mut self, props: HashMap<String, TokenStream>) -> &mut Self {
        self.props = Some(props);
        self
    }
    pub fn push_prop(&mut self, key: String, value: TokenStream) -> &mut Self {
        if self.props.is_none() {
            self.props.replace(HashMap::new());
        }
        self.props.as_mut().unwrap().insert(key, value);

        self
    }
    pub fn set_events(&mut self, events: HashMap<String, TokenStream>) -> &mut Self {
        self.events = Some(events);
        self
    }
    pub fn push_event(&mut self, key: String, value: TokenStream) -> &mut Self {
        if self.events.is_none() {
            self.events.replace(HashMap::new());
        }
        self.events.as_mut().unwrap().insert(key, value);

        self
    }
    pub fn set_prop_ptr(&mut self, prop_ptr: TokenStream) -> &mut Self {
        self.prop_ptr = Some(prop_ptr);
        self
    }
    pub fn set_event_ptr(&mut self, event_ptr: TokenStream) -> &mut Self {
        self.event_ptr = Some(event_ptr);
        self
    }
    pub fn set_children(&mut self, children: Vec<Widget>) -> &mut Self {
        self.children = Some(children);
        self
    }
    pub fn push_child(&mut self, child: Widget) -> &mut Self {
        if self.children.is_none() {
            self.children.replace(vec![]);
        }
        self.children.as_mut().unwrap().push(child);

        self
    }
    pub fn set_inherits(&mut self, inherits: BuiltIn) -> &mut Self {
        self.inherits = Some(inherits);
        self
    }
    pub fn set_traits(&mut self, traits: WidgetTrait) -> &mut Self {
        self.traits = traits;
        self
    }
    pub fn set_role(&mut self, role: Role) -> &mut Self {
        self.role = role;
        self
    }
}
