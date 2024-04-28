use std::{collections::HashMap, path::PathBuf};

use gen_converter::model::{prop::ConvertStyle, TemplateModel};
use gen_parser::{PropsKey, Value};
use gen_utils::common::snake_to_camel;
use proc_macro2::{TokenStream, TokenTree};

use crate::widget::{self, BuiltIn, StaticProps};

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
    pub source: Option<String>,
    pub compiled_source: Option<PathBuf>,
    /// props in live_design
    pub props: Option<TokenStream>,
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
    pub fn new(special: &str, mut source_dir: PathBuf) -> Self {
        let mut widget = Widget::default();
        widget.source = Some(special.to_string());
        // 获取文件名且改为首字母大写的camel
        widget.name = snake_to_camel(&special.split("/").last().unwrap().replace(".gen", ""))
            .expect("can not transfer to camel");

        source_dir.pop();
        source_dir.push("src-gen");

        widget.compiled_source = Some(source_dir);
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
    pub fn set_in_live_design(&mut self, in_live_design: bool) -> &mut Self {
        self.in_live_design = in_live_design;
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
            self.props = Some(BuiltIn::from(&self.name).props(&props));
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

impl From<gen_converter::model::Model> for Widget {
    fn from(value: gen_converter::model::Model) -> Self {
        let gen_converter::model::Model {
            special,
            template,
            script,
            style,
            compile,
            is_entry,
            source,
        } = value;

        let template = template.unwrap();

        dbg!(&template);

        let mut widget = if template.get_name().eq("component") {
            let mut widget = Widget::new(&special, source);
            let _ = widget.set_inherits(BuiltIn::from(template.get_inherits().unwrap()));
            widget
        } else {
            build_widget(&template, style.as_ref())
        };

        if template.has_children() {
            widget.set_children(
                template
                    .get_children()
                    .unwrap()
                    .iter()
                    .map(|item| build_widget(item, style.as_ref()))
                    .collect(),
            );
        }

        todo!("{:#?}", widget);
    }
}

fn build_widget(template: &TemplateModel, style: Option<&ConvertStyle>) -> Widget {
    let mut widget = Widget::new_builtin(template.get_name());
    // get styles from style by id
    let widget_styles = get_widget_styles(template.get_id(), style);
    let widget_styles = combine_styles(widget_styles, template.get_unbind_props());
    widget
        .set_is_root(template.is_root())
        .set_id(template.get_id())
        .set_props(widget_styles);

    if template.has_children() {
        widget.set_children(
            template
                .get_children()
                .unwrap()
                .iter()
                .map(|item| build_widget(item, style))
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

fn combine_styles(l: Option<HashMap<PropsKey, Value>>, r: Option<HashMap<&PropsKey, &Value>>) -> Option<HashMap<PropsKey, Value>> {
    match (l, r) {
        (Some(l), Some(r)) => {
            let mut styles = l.clone();
            for (k, v) in r {
                styles.insert(k.clone(), v.clone());
            }
            Some(styles)
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r.into_iter().map(|(k,v)| (k.clone(),v.clone())).collect()),
        (None, None) => None,
    }
}
