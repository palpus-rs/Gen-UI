use std::fmt::{write, Display};

use super::PropRole;

#[derive(Debug, Clone, PartialEq)]
pub struct MakepadModel {
    special: Option<String>,
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
        }
    }
    pub fn set_special(&mut self, special: String) {
        if !special.is_empty() {
            self.special.replace(special);
        }
    }
    pub fn has_special(&self) -> bool {
        self.special.is_some()
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
        let _ = f.write_fmt(format_args!("<{}>", &self.tag));
        // add props
        let _ = 
        write!(f, "")
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
    use crate::targets::makepad::{value::MakepadPropValue, PropRole};

    use super::MakepadModel;

    #[test]
    fn test_display() {
        let mut model = MakepadModel::new("Window", true);
        model.set_special("my_ui".to_string());
        model.push_prop(PropRole::Special(String::from("my_ui")));
        model.push_prop(PropRole::Normal(
            "height".to_string(),
            MakepadPropValue::F64(180.0),
        ));

        dbg!(model.to_string());
    }
}
