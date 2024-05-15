use std::path::PathBuf;

use gen_converter::model::{Model, Source};
use proc_macro2::TokenStream;

use crate::{
    widget::model::{widget::Widget, ToLiveDesign},
    ToToken,
};

use super::RsFile;

#[derive(Debug, Clone)]
pub enum ModelNode {
    Widget(Widget),
    RsFile(RsFile),
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
}

impl From<Model> for ModelNode {
    fn from(value: Model) -> Self {
        let source = &value.special;
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
