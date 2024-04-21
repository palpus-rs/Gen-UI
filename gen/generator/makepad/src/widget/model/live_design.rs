use proc_macro2::TokenStream;

use crate::utils::{import_makepad_widgets_base, import_makepad_widgets_theme_desktop_dark};

use super::widget::Widget;

/// LiveDesign中包含了Makepad DSL的模板部分，这个部分是必须的
/// 它由大量虚拟Widget节点组成
#[derive(Debug, Clone)]
pub struct LiveDesign {
    /// live design 中引入的依赖
    imports: TokenStream,
    /// live design 中的节点树
    tree: Option<Box<Widget>>,
}

impl Default for LiveDesign {
    fn default() -> Self {
        let mut imports = TokenStream::new();
        imports.extend(import_makepad_widgets_base());
        imports.extend(import_makepad_widgets_theme_desktop_dark());

        Self {
            imports,
            tree: None,
        }
    }
}
