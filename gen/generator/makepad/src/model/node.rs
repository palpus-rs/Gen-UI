use std::{hash::Hash, io::Write, path::PathBuf};

use gen_converter::model::{Model, Source};
use proc_macro2::TokenStream;

use crate::{
    utils::create_file,
    widget::model::{widget::Widget, ToLiveDesign},
    ToToken,
};

use super::RsFile;

#[derive(Debug, Clone)]
pub enum ModelNode {
    Widget(Widget),
    RsFile(RsFile),
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.source().unwrap() == other.source().unwrap()
    }
}

impl Eq for ModelNode {}

impl Hash for ModelNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.source().unwrap().hash(state);
    }
}

impl ModelNode {
    pub fn source(&self) -> Option<&Source> {
        match self {
            ModelNode::Widget(widget) => widget.source.as_ref(),
            ModelNode::RsFile(rs) => Some(&rs.source),
        }
    }
    pub fn content(&self) -> TokenStream {
        match self {
            ModelNode::Widget(widget) => widget.to_live_design().to_token_stream(),
            ModelNode::RsFile(rs) => rs.content(),
        }
    }
    pub fn level(&self) -> (usize, PathBuf) {
        let path = self.source().unwrap().level_gen();
        (path.components().count(), path)
    }
    pub fn super_ui_root(&self) -> (String, String) {
        match self {
            ModelNode::Widget(widget) => {
                let root = widget
                    .source
                    .as_ref()
                    .expect("first ui root need source")
                    .source_name_lower();
                (root, widget.id.as_ref().unwrap().to_string())
            }
            ModelNode::RsFile(_) => panic!("super ui root not exist in rs file"),
        }
    }
    pub fn compile(&self) -> () {
        let content = self.content().to_string();
        let mut file = create_file(self.source().unwrap().compiled_file.as_path());
        file.write_all(content.as_bytes()).unwrap();
    }
}

impl From<Model> for ModelNode {
    fn from(value: Model) -> Self {
        let source = &value.special;
        // dbg!(&value);
        match &value.strategy {
            gen_parser::Strategy::None => RsFile::new_empty(source.clone()).into(),
            gen_parser::Strategy::SingleScript => RsFile::from(value).into(),
            gen_parser::Strategy::Error(e) => panic!("{}", e),
            _ => Widget::from(value).into(),
        }
    }
}

impl From<Widget> for ModelNode {
    fn from(value: Widget) -> Self {
        ModelNode::Widget(value)
    }
}

impl From<RsFile> for ModelNode {
    fn from(value: RsFile) -> Self {
        ModelNode::RsFile(value)
    }
}

#[cfg(test)]
mod test_node {
    use crate::model::ModelTree;

    use super::*;
    #[test]
    fn test_eq() {
        let source = Source::from((
            "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\a.gen",
            "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
        ));
        let node1 = ModelNode::Widget(Widget::new(
            Some(source.clone()),
            "hello",
            Some(&"view".to_string()),
        ));
        let node2 = ModelNode::RsFile(RsFile::new(source, TokenStream::new()));

        assert_eq!(node1, node2);
    }
    #[test]
    fn test_eq_tree() {
        let source = Source::from((
            "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui\\a.gen",
            "E:\\Rust\\try\\makepad\\Gen-UI\\examples\\simple1\\ui",
        ));
        let node1 = ModelNode::Widget(Widget::new(
            Some(source.clone()),
            "hello",
            Some(&"view".to_string()),
        ));
        let node2 = ModelNode::RsFile(RsFile::new(source, TokenStream::new()));
        let default_node = ModelNode::Widget(Widget::default_ui_root());
        let mut tree = ModelTree::new(default_node);
        tree.children = Some(std::iter::once(ModelTree::from(node1)).collect());
        tree.add(node2);

        dbg!(tree);
    }
}
