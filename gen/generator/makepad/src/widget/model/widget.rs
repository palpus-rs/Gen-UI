use std::collections::HashMap;

use gen_converter::model::{
    prop::ConvertStyle,
    script::{GenScriptModel, ScriptModel, UseMod},
    Source, TemplateModel,
};
use gen_parser::{PropsKey, Value};

use proc_macro2::{TokenStream, TokenTree};
use syn::{ItemEnum, ItemStruct, Stmt};

use crate::widget::{BuiltIn, StaticProps};

use super::{handler::WidgetHandler, live_design::LiveDesign, role::Role, traits::WidgetTrait};

/// ## 当生成 live_design! 中的节点时
/// `[id] [:|=] <name>{ [...props|widget...] }`
/// ## 当生成一个完整的组件时
#[derive(Debug, Default, Clone)]
pub struct Widget {
    /// Makepad live_design! macro
    pub live_design: Option<LiveDesign>,
    pub is_root: bool,
    pub is_prop: bool,
    pub is_built_in: bool,
    /// widget id, if widget is prop, id is prop
    pub id: Option<String>,
    pub name: String,
    pub source: Option<Source>,
    pub uses: Option<TokenStream>,
    // pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<TokenStream>,
    /// events called in makepad
    pub events: Option<HashMap<String, TokenStream>>,
    pub prop_ptr: Option<TokenStream>,
    pub event_ptr: Option<TokenStream>,
    pub event_ref: Option<TokenStream>,
    pub event_set: Option<TokenStream>,
    pub children: Option<Vec<Widget>>,
    pub inherits: Option<BuiltIn>,
    pub traits: WidgetTrait,
    pub role: Role,
}

impl Widget {
    pub fn new(special: Option<Source>, name: &str) -> Self {
        let mut widget = Widget::default();
        match special {
            Some(special) => {
                widget.source.replace(special);
                // 获取文件名且改为首字母大写的camel
                widget.name = widget.source.as_ref().unwrap().source_name();
                widget.set_inherits(BuiltIn::try_from(name).unwrap());
            }
            None => {
                widget.name = name.to_string();
                let builtin = BuiltIn::try_from(name);
                widget
                    .set_is_built_in(builtin.is_ok())
                    .set_inherits(builtin.unwrap());
            }
        }
        widget
    }
    pub fn new_builtin(name: &str) -> Self {
        let mut widget = Widget::default();
        widget.name = name.to_string();
        widget.set_is_built_in(BuiltIn::try_from(name).is_ok());
        widget
    }
    pub fn set_id(&mut self, id: Option<&String>) -> &mut Self {
        if let Some(id) = id {
            self.id = Some(id.to_string());
        }
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
    pub fn set_is_built_in(&mut self, is_built_in: bool) -> &mut Self {
        self.is_built_in = is_built_in;
        self
    }
    /// if can not parse by BuiltIn Widget -> panic!
    pub fn set_props(&mut self, props: Option<HashMap<PropsKey, Value>>) -> &mut Self {
        if let Some(props) = props {
            // if self.is_built_in {
            //     self.props = Some(BuiltIn::from(&self.name).props(&props));
            // } else {
            //     todo!("widget props define unsoloved => {:#?}",props);
            // }
            if self.is_built_in {
                self.props = Some(BuiltIn::from(&self.name).props(&props));
            }
        }
        self
    }
    // pub fn push_prop(&mut self, key: String, value: TokenStream) -> &mut Self {
    //     if self.props.is_none() {
    //         self.props.replace(HashMap::new());
    //     }
    //     self.props.as_mut().unwrap().insert(key, value);

    //     self
    // }
    /// - set prop_ptr
    /// - set event_ptr
    /// - set lifetime
    /// - set
    pub fn set_script(&mut self, script: Option<&ScriptModel>) -> &mut Self {
        if let Some(sc) = script {
            if let ScriptModel::Gen(sc) = sc {
                let GenScriptModel {
                    uses,
                    prop_ptr,
                    event_ptr,
                    sub_prop_binds,
                    sub_event_binds,
                    other,
                    ..
                } = sc;

                self.set_uses(uses)
                    .set_prop_ptr(prop_ptr)
                    .set_event_ptr(event_ptr)
                    .draw_walk(other);

                todo!("{:#?}", self);
            }
        }
        self
    }
    pub fn draw_walk(&mut self, sc: &Option<Vec<Stmt>>) -> &mut Self {
        if let Some(stmts) = sc {
            let _ = self.traits.draw_walk(stmts);
        }
        self
    }
    pub fn set_uses(&mut self, uses: &Option<UseMod>) -> &mut Self {
        if let Some(uses) = uses {
            self.uses = WidgetHandler::uses(uses);
        }
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
    pub fn get_inherits(&self) -> Option<&BuiltIn> {
        self.inherits.as_ref()
    }
    pub fn set_prop_ptr(&mut self, prop_ptr: &Option<ItemStruct>) -> &mut Self {
        if let Some(prop_ptr) = prop_ptr {
            self.prop_ptr.replace(WidgetHandler::prop_ptr(
                prop_ptr,
                self.get_inherits().unwrap(),
            ));
        }
        self
    }
    pub fn set_event_ptr(&mut self, event_ptr: &Option<ItemEnum>) -> &mut Self {
        if let Some(event_ptr) = event_ptr {
            self.event_ptr.replace(WidgetHandler::event_ptr(event_ptr));
        }

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

impl From<gen_converter::model::Model> for Widget {
    fn from(value: gen_converter::model::Model) -> Self {
        let gen_converter::model::Model {
            special,
            template,
            script,
            style,
            compile,
            is_entry,
        } = value;

        let template = template.unwrap();

        // dbg!(&template);
        // dbg!(&script);

        let mut widget = build_widget(Some(special), &template, style.as_ref(), script.as_ref());

        // todo!();
        todo!("{:#?}", widget);
    }
}

fn build_widget(
    special: Option<Source>,
    template: &TemplateModel,
    style: Option<&ConvertStyle>,
    script: Option<&ScriptModel>,
) -> Widget {
    let mut widget = Widget::new(special, template.get_name());
    // get styles from style by id
    let widget_styles = get_widget_styles(template.get_id(), style);
    let widget_styles = combine_styles(widget_styles, template.get_unbind_props());
    widget
        .set_is_root(template.is_root())
        .set_id(template.get_id())
        .set_props(widget_styles)
        .set_script(script);

    if template.has_children() {
        widget.set_children(
            template
                .get_children()
                .unwrap()
                .iter()
                .map(|item| build_widget(None, item, style, None))
                .collect(),
        );
    }
    return widget;
}

/// get styles from style by id
fn get_widget_styles(
    id: Option<&String>,
    styles: Option<&ConvertStyle>,
) -> Option<HashMap<PropsKey, Value>> {
    match styles {
        Some(styles) => match id {
            Some(id) => match styles.get(id) {
                Some(style) => Some(style.clone()),
                None => None,
            },
            None => None,
        },
        None => None,
    }
}

fn combine_styles(
    l: Option<HashMap<PropsKey, Value>>,
    r: Option<HashMap<&PropsKey, &Value>>,
) -> Option<HashMap<PropsKey, Value>> {
    match (l, r) {
        (Some(l), Some(r)) => {
            let mut styles = l.clone();
            for (k, v) in r {
                styles.insert(k.clone(), v.clone());
            }
            Some(styles)
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r.into_iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
        (None, None) => None,
    }
}
