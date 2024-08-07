use proc_macro2::TokenStream;
use quote::quote;

use crate::ToToken;

use super::ToLiveDesign;

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

        let imports = quote! {
            import makepad_widgets::base::*;
            import makepad_widgets::theme_desktop_dark::*;
            import makepad_draw::shader::std::*;
        };

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

impl<T> From<&T> for LiveDesign
where
    T: ToLiveDesign,
{
    fn from(value: &T) -> Self {
        let mut live_design = LiveDesign::default();

        let tree = value.widget_tree();
        let logic = value.widget_logic();
        let imports = value.widget_imports();

        live_design.tree = tree;
        live_design.logic = logic;
        if let Some(imports) = imports {
            live_design.imports.extend(imports);
        }

        live_design
    }
}
