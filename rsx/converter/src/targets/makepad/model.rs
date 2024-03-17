use std::fmt::Display;

use crate::context::{LEFT_HOLDER, RIGHT_HOLDER};

use super::PropRole;

/// # The Model of Makepad
/// Model includes all built-in widgets
#[derive(Debug, Clone, PartialEq)]
pub struct MakepadModel {
    special: Option<String>,
    contexts: Option<Vec<String>>,
    tag: String,
    props: Option<Vec<PropRole>>,
    children: Option<Vec<MakepadModel>>,
    is_ref: bool,
}

impl MakepadModel {
    pub fn new(tag: &str, is_ref: bool) -> Self {
        MakepadModel {
            special: None,
            tag: tag.to_string(),
            props: None,
            children: None,
            is_ref,
            contexts: None,
        }
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
        match self.tag.as_str() {
            "Window" | "View" => self
                .props
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|prop| prop.to_string())
                .collect::<String>(),
            "Label" => String::new(),
            _ => panic!("Invalid widget"),
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
        } else {
        }
        // add tag
        let _ = f.write_fmt(format_args!("<{}>{}", &self.tag, LEFT_HOLDER));
        // add props

        if self.has_props() {
            let props = self
                .props
                .as_ref()
                .unwrap()
                .into_iter()
                .map(|prop| prop.to_string())
                .collect::<String>();

            let _ = f.write_str(&props);
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

        f.write_str(RIGHT_HOLDER)
    }
}

pub fn models_to_string(models: Vec<MakepadModel>) -> String {
    models
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod test_mk_model {
    use crate::targets::makepad::{
        value::{MakepadPropValue, Size},
        PropRole,
    };

    use super::MakepadModel;

    #[test]
    fn test_display() {
        let mut model = MakepadModel::new("Window", true);
        model.set_special("my_ui".to_string());

        model.push_prop(PropRole::Normal(
            "height".to_string(),
            MakepadPropValue::Size(Size::Fixed(180.0)),
        ));

        model.push_prop(PropRole::Normal(
            "width".to_string(),
            MakepadPropValue::Size(Size::Fill),
        ));

        model.push_child(MakepadModel::new("Button", false));

        let mut nesting = MakepadModel::new("View", false);
        nesting.push_child(MakepadModel::new("Button", false));
        model.push_child(nesting);

        dbg!(model.to_string());
    }
}
