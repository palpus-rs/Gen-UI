use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    utils::{import_makepad_widgets_base, import_makepad_widgets_theme_desktop_dark},
    ToToken,
};

use super::widget::Widget;

/// LiveDesign中包含了Makepad DSL的模板部分，这个部分是必须的
/// 它由大量虚拟Widget节点组成
#[derive(Debug, Clone)]
pub struct LiveDesign {
    /// live design 中引入的依赖
    pub imports: TokenStream,
    /// live design 中的节点树
    pub tree: Option<TokenStream>,
    pub logic: Option<TokenStream>,
}

impl LiveDesign {
    pub fn set_tree(&mut self) -> () {
        // self.tree.replace()
    }
}

impl Default for LiveDesign {
    fn default() -> Self {
        let mut imports = TokenStream::new();
        imports.extend(import_makepad_widgets_base());
        imports.extend(import_makepad_widgets_theme_desktop_dark());

        Self {
            imports,
            tree: None,
            logic: None,
        }
    }
}

impl ToToken for LiveDesign {
    fn to_token_stream(&self) -> TokenStream {
        let imports = &self.imports;
        let tree = &self.tree;
        let logic = &self.logic;

        quote! {
            use makepad_widgets::*;
            live_design!{
                #imports

                #tree
            }

            #logic
        }
    }
}

impl From<Widget> for LiveDesign {
    fn from(value: Widget) -> Self {
        let mut live_design = LiveDesign::default();

        let tree = value.widget_tree();
        let logic = value.widget_logic();

        live_design.tree = tree;
        live_design.logic = logic;

        live_design
    }
}
