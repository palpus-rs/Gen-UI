use std::{collections::HashMap, default, fmt::Display};

use gen_parser::{PropsKey, Value};
use gen_utils::common::{
    snake_to_camel, token_stream_to_tree, token_tree_ident, token_tree_punct_alone,
    trees_to_token_stream,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::TokenStreamExt;

use crate::utils::{apply_over_and_redraw, live_macro, struct_field_type};

pub mod button;
pub mod define;
pub mod label;
pub mod view;
pub mod window;

// pub use define::*;
// pub use button::*;
// pub use label::*;
// pub use view::*;
// pub use window::*;

const WINDOW: &str = "Window";
const VIEW: &str = "View";
const LABEL: &str = "Label";
const BUTTON: &str = "Button";

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Widget {
    Window,
    #[default]
    View,
    Label,
    Button,
    Define(String),
}

impl Widget {
    pub fn ast(&self) {
        match self {
            Widget::Define(name) => todo!(),
            _ => todo!(),
        }
    }
    pub fn props(&self, props: HashMap<&PropsKey, &Value>) -> Vec<TokenTree> {
        let mut ast = vec![];
        props.iter().for_each(|(prop, value)| {
            let prop_name = prop.name();
            let prop_value = value.is_unknown_and_get().unwrap();
            ast.extend(match self {
                Widget::Window => todo!(),
                Widget::View => view::prop(prop_name, prop_value),
                Widget::Label => todo!(),
                Widget::Button => todo!(),
                Widget::Define(_) => todo!(),
            });
        });
        ast
    }
    pub fn props_from_tk(
        &self,
        tag: String,
        id: String,
        pvs: Vec<(PropsKey, String, TokenStream, bool)>,
    ) -> (TokenStream, TokenStream, TokenStream, Vec<TokenTree>) {
        let mut prop_fts = TokenStream::new();
        let mut props = TokenStream::new();
        let mut codes = TokenStream::new();
        let mut fields = TokenStream::new();
        pvs.into_iter().for_each(|(k, ident, code, _)| {
            let (p_tk, ty_tk) = self.prop_from_str(&k, &ident.as_str());
            props.extend(p_tk);
            prop_fts.extend(struct_field_type(&ident, ty_tk));
            codes.extend(code);
            fields.extend(vec![token_tree_ident(&ident), token_tree_punct_alone(',')]);
        });
        (
            prop_fts,
            codes,
            fields,
            apply_over_and_redraw(None, tag, id, token_stream_to_tree(props)),
        )
    }
    fn prop_from_str(&self, k: &PropsKey, v: &str) -> (Vec<TokenTree>, TokenTree) {
        let prop_name = k.name();
        match self {
            Widget::Window => todo!(),
            Widget::View => view::prop_token(prop_name, v),
            Widget::Label => todo!(),
            Widget::Button => todo!(),
            Widget::Define(_) => todo!(),
        }
    }
}

impl From<&str> for Widget {
    fn from(value: &str) -> Self {
        let widget_name = snake_to_camel(value).unwrap();
        match widget_name.as_str() {
            WINDOW => Widget::Window,
            VIEW => Widget::View,
            LABEL => Widget::Label,
            BUTTON => Widget::Button,
            _ => Widget::Define(widget_name),
        }
    }
}

impl Display for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Widget::Window => WINDOW,
            Widget::View => VIEW,
            Widget::Label => LABEL,
            Widget::Button => BUTTON,
            Widget::Define(d) => d,
        })
    }
}
