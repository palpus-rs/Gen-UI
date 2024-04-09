//! 构建Makepad live_design!{}宏的TokenStream

use gen_converter::model::TemplateModel;
use gen_utils::common::snake_to_camel;
use proc_macro2::TokenStream;

use crate::{utils::{component_render, import_makepad_widgets_base, import_makepad_widgets_theme_desktop_dark, live_design_macro, special_struct, use_makepad_widget_all}, widget::Widget};

pub fn template(special:&str,t: Option<&TemplateModel>) -> TokenStream {
    let mut tk = TokenStream::new();
    if let Some(t) = t {

        let id = t.get_id();
        let is_root = t.is_root();
        let is_component = t.is_component();
        let tag = snake_to_camel(t.get_name()).unwrap();
        let props = match t.get_props(){
            Some(props) => {
                let widget = Widget::from(tag.as_str());
                Some(widget.props(props))
            },
            None => None,
        };
        

        tk.extend(use_makepad_widget_all());
        // import --------------------------------------------------------------------------------
        let mut live_inner = import_makepad_widgets_base();
        live_inner.extend(import_makepad_widgets_theme_desktop_dark());
        // special ------------------------------------------------------------------------------
        let s = snake_to_camel(&special.split("/").last().unwrap().replace(".gen", ""))
                .expect("can not transfer to camel");
        live_inner.extend(special_struct(&s, component_render(id, is_root, is_component, &tag, props)));
        // live_design ---------------------------------------------------------------------------
        let live_design = live_design_macro(
            live_inner,
        );
        tk.extend(live_design);
    }
    tk
}
