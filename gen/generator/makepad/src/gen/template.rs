//! 构建Makepad live_design!{}宏的TokenStream

use gen_converter::model::TemplateModel;
use gen_utils::common::snake_to_camel;
use proc_macro2::TokenStream;

use crate::utils::{import_makepad_widgets_base, import_makepad_widgets_theme_desktop_dark, live_design_macro, special_struct, use_makepad_widget_all};

pub fn template(special:&str,t: Option<&TemplateModel>) -> TokenStream {
    let mut tk = TokenStream::new();
    if let Some(t) = t {
        tk.extend(use_makepad_widget_all());
        // import --------------------------------------------------------------------------------
        let mut live_inner = import_makepad_widgets_base();
        live_inner.extend(import_makepad_widgets_theme_desktop_dark());
        // special ------------------------------------------------------------------------------
        let s = snake_to_camel(&special.split("/").last().unwrap().replace(".gen", ""))
                .expect("can not transfer to camel");
        live_inner.extend(special_struct(&s, code));
        // live_design ---------------------------------------------------------------------------
        let live_design = live_design_macro(
            live_inner,
        );
        tk.extend(live_design);
    }
    tk
}
