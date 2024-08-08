use std::collections::HashMap;

use gen_utils::common::Source;

use crate::{compiler::AUTO_BUILTIN_WIDGETS, widget::BuiltIn};

use super::{
    role::Role,
    safe_traits::{SafeLiveHookTrait, SafeWidgetTrait},
    widget::Widget,
};

/// copy from Widget struct but it need to be used in lazy_static: AUTO_BUILTIN_WIDGETS
/// so this struct should be sync + send + safe
/// It replace String to String
#[derive(Debug, Default, Clone)]
pub struct SafeWidget {
    pub is_root: bool,
    pub is_prop: bool,
    pub is_built_in: bool,
    /// is a define widget
    pub is_static: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    /// is widget as a prop? if prop is true , widget need id
    /// `<view id="a" as_prop></view>` => as_prop = true
    pub as_prop: bool,
    pub name: String,
    pub source: Option<Source>,
    pub imports: Option<String>,
    pub uses: Option<String>,
    // pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<String>,
    /// events called in makepad
    pub events: Option<HashMap<String, String>>,
    pub prop_ptr: Option<String>,
    pub event_ptr: Option<String>,
    pub event_ref: Option<String>,
    pub event_set: Option<String>,
    pub children: Option<Vec<SafeWidget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: Option<SafeWidgetTrait>,
    pub live_hook: Option<SafeLiveHookTrait>,
    // still need, maybe the for | if widgets are nested
    pub role: Role,
    /// the widget tree code, it should be set when SafeWidget is created (from Widget::handle_role()!!!)
    pub tree: Option<String>,
}

impl SafeWidget {
    pub fn insert_to_auto(self) -> () {
        let mut auto_widgets = AUTO_BUILTIN_WIDGETS.lock().unwrap();
        auto_widgets.push(self);
    }
    pub fn to_live_import(&self) -> String {
        let id = match &self.role {
            Role::If { id, .. } => id,
            Role::For { id, .. } => id,
            Role::Normal => panic!("normal widget not need to transform to safe widget!"),
        };

        format!("import crate::auto::{}_{}::*;", self.name, id)
    }
}

impl From<&Widget> for SafeWidget {
    fn from(value: &Widget) -> Self {
        let Widget {
            is_root,
            is_prop,
            is_built_in,
            is_static,
            id,
            as_prop,
            name,
            source,
            imports,
            uses,
            props,
            events,
            prop_ptr,
            event_ptr,
            event_ref,
            event_set,
            children,
            inherits,
            traits,
            live_hook,
            role,
        } = value;

        SafeWidget {
            is_root: *is_root,
            is_prop: *is_prop,
            is_built_in: *is_built_in,
            is_static: *is_static,
            id: id.clone(),
            as_prop: *as_prop,
            name: name.clone(),
            source: source.clone(),
            imports: imports.as_ref().map(|x| x.to_string()),
            uses: uses.as_ref().map(|x| x.to_string()),
            props: props.as_ref().map(|x| x.to_string()),
            events: events.as_ref().map(|x| {
                x.into_iter()
                    .map(|(k, v)| (k.clone(), v.to_string()))
                    .collect()
            }),
            prop_ptr: prop_ptr.as_ref().map(|x| x.to_string()),
            event_ptr: event_ptr.as_ref().map(|x| x.to_string()),
            event_ref: event_ref.as_ref().map(|x| x.to_string()),
            event_set: event_set.as_ref().map(|x| x.to_string()),
            children: children
                .as_ref()
                .map(|x| x.iter().map(|item| item.into()).collect()),
            inherits: inherits.clone(),
            traits: traits.as_ref().map(|x| x.into()),
            live_hook: live_hook.as_ref().map(|x| x.into()),
            role: role.clone(),
            tree: None,
        }
    }
}
