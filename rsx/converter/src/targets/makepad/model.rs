use std::fmt::Display;

use crate::context::{LEFT_HOLDER, RIGHT_HOLDER};

use super::{generate_label_props, PropRole, Widgets};

/// # The Model of Makepad
/// Model includes all built-in widgets
#[derive(Debug, Clone, PartialEq)]
pub struct MakepadModel {
    special: Option<String>,
    contexts: Option<Vec<String>>,
    tag: String,
    props: Option<Vec<PropRole>>,
    actions: Option<Vec<PropRole>>,
    children: Option<Vec<MakepadModel>>,
    is_ref: bool,
    inherits: Option<Widgets>,
}

impl MakepadModel {
    pub fn new(tag: &str, is_ref: bool) -> Self {
        MakepadModel {
            special: None,
            tag: tag.to_string(),
            props: None,
            actions: None,
            children: None,
            is_ref,
            inherits: None,
            contexts: None,
        }
    }
    pub fn is_inherit(&self) -> bool {
        self.inherits.is_some()
    }
    pub fn set_inherit(&mut self, inherits: Option<Widgets>) {
        self.inherits = inherits;
    }
    pub fn get_inherit(&self) -> Option<&Widgets> {
        self.inherits.as_ref()
    }
    pub fn get_contexts(&self) -> Option<&Vec<String>> {
        self.contexts.as_ref()
    }
    pub fn set_contexts(&mut self, context: &Vec<String>) -> () {
        self.contexts.replace(context.clone());
    }
    pub fn push_context(&mut self, item: String) {
        match &mut self.contexts {
            Some(c) => {
                let _ = c.push(item);
            }
            None => {
                let _ = self.contexts.replace(vec![item]);
            }
        };
    }
    pub fn push_context_ref(&mut self, item: &str) {
        self.push_context(item.to_string())
    }
    pub fn has_contexts(&self) -> bool {
        self.contexts.is_some()
    }
    pub fn has_links(&self) -> bool {
        self.has_contexts() || self.has_special()
    }
    pub fn get_links(&self) -> Option<Vec<String>> {
        match (self.has_special(), self.has_contexts()) {
            (true, true) => {
                let mut res = self.get_contexts().unwrap().clone();
                res.push(self.get_special().unwrap().to_string());
                Some(res)
            }
            (true, false) => Some(vec![self.get_special().unwrap().to_string()]),
            (false, true) => Some(self.get_contexts().unwrap().clone()),
            (false, false) => None,
        }
    }
    pub fn get_special(&self) -> Option<&String> {
        self.special.as_ref()
    }
    pub fn set_special(&mut self, special: String) {
        if !special.is_empty() {
            self.special.replace(special);
        }
    }
    pub fn has_special(&self) -> bool {
        self.special.is_some()
    }
    pub fn get_props(&self) -> Option<&Vec<PropRole>> {
        self.props.as_ref()
    }
    pub fn get_bind_props(&self) -> Option<Vec<PropRole>> {
        if let Some(props) = self.get_props() {
            let mut res = vec![];
            for prop in props {
                match prop {
                    PropRole::Bind(_, _) => {
                        res.push(prop.clone());
                    }
                    _ => {}
                }
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        } else {
            None
        }
    }
    pub fn has_props(&self) -> bool {
        self.props.is_some()
    }
    pub fn push_prop(&mut self, item: PropRole) -> () {
        match &mut self.props {
            Some(props) => props.push(item),
            None => {
                let _ = self.props.replace(vec![item]);
            }
        };
    }
    pub fn set_props(&mut self, props: Vec<PropRole>) -> () {
        let _ = self.props.replace(props);
    }
    pub fn push_child(&mut self, item: MakepadModel) -> () {
        match &mut self.children {
            Some(children) => children.push(item),
            None => {
                let _ = self.children.replace(vec![item]);
            }
        }
    }
    pub fn set_children(&mut self, children: Vec<MakepadModel>) -> () {
        let _ = self.children.replace(children);
    }
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
    fn props_to_string(&self) -> String {
        let props = self.props.as_ref().unwrap();
        props_to_string(self.tag.as_str(), props)
    }
    pub fn push_action(&mut self, item: PropRole) -> () {
        // let item = PropRole::Function(action);
        match &mut self.actions {
            Some(actions) => actions.push(item),
            None => {
                let _ = self.actions.replace(vec![item]);
            }
        }
    }
}

impl Display for MakepadModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_special() {
            // ref tag
            // `special: <tag_name>{}`
            if self.is_ref {
                let _ = f.write_fmt(format_args!("{}: ", self.special.as_ref().unwrap()));
            } else {
                // unref tag
                // `special = <tag_name>{}`
                let _ = f.write_fmt(format_args!("{} = ", self.special.as_ref().unwrap()));
            }
        }
        // add tag
        if !self.is_inherit() {
            let _ = f.write_fmt(format_args!("<{}>{}", &self.tag, LEFT_HOLDER));
            // add props
            if self.has_props() {
                let props = self.props_to_string();
                // dbg!(&props);

                let _ = f.write_str(&props);
            }
        }
        // add children
        if self.has_children() {
            let children = self
                .children
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|child| child.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            let _ = f.write_fmt(format_args!(" {} ", &children));
        }
        if !self.is_inherit(){
            f.write_str(RIGHT_HOLDER)
        }else{
            f.write_str("")
        }
        
    }
}

pub fn models_to_string(models: Vec<MakepadModel>) -> String {
    models
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

pub fn props_to_string(tag: &str, props: &Vec<PropRole>) -> String {
    match tag {
        "Window" | "View" | "Button" => props
            .into_iter()
            .map(|prop| prop.to_string())
            .collect::<String>(),
        "Label" => generate_label_props(props),
        _ => panic!("Invalid widget"),
    }
}