use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident};

use super::{live_hook::LiveHookTrait, traits::WidgetTrait};

#[derive(Debug, Default, Clone)]
pub struct SafeWidgetTrait {
    pub draw_walk: String,
    pub handle_event: Option<String>,
    pub widget: Option<String>,
    pub widgets: Option<String>,
    pub widget_id: Option<String>,
    pub widget_to_data: Option<String>,
    pub data_to_widget: Option<String>,
    pub draw: Option<String>,
    pub draw_walk_all: Option<String>,
    pub is_visible: Option<String>,
    pub draw_all: Option<String>,
    pub text: Option<String>,
    pub set_text: Option<String>,
    pub set_text_and_redraw: Option<String>,
    pub ref_cast_type_id: Option<String>,
}

impl From<&WidgetTrait> for SafeWidgetTrait {
    fn from(value: &WidgetTrait) -> Self {
        let WidgetTrait {
            draw_walk,
            handle_event,
            widget,
            widgets,
            widget_id,
            widget_to_data,
            data_to_widget,
            draw,
            draw_walk_all,
            is_visible,
            draw_all,
            text,
            set_text,
            set_text_and_redraw,
            ref_cast_type_id,
        } = value;

        Self {
            draw_walk: draw_walk.to_string(),
            handle_event: handle_event.as_ref().map(|x| x.to_string()),
            widget: widget.as_ref().map(|x| x.to_string()),
            widgets: widgets.as_ref().map(|x| x.to_string()),
            widget_id: widget_id.as_ref().map(|x| x.to_string()),
            widget_to_data: widget_to_data.as_ref().map(|x| x.to_string()),
            data_to_widget: data_to_widget.as_ref().map(|x| x.to_string()),
            draw: draw.as_ref().map(|x| x.to_string()),
            draw_walk_all: draw_walk_all.as_ref().map(|x| x.to_string()),
            is_visible: is_visible.as_ref().map(|x| x.to_string()),
            draw_all: draw_all.as_ref().map(|x| x.to_string()),
            text: text.as_ref().map(|x| x.to_string()),
            set_text: set_text.as_ref().map(|x| x.to_string()),
            set_text_and_redraw: set_text_and_redraw.as_ref().map(|x| x.to_string()),
            ref_cast_type_id: ref_cast_type_id.as_ref().map(|x| x.to_string()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct SafeLiveHookTrait {
    pub before_live_design: Option<String>,
    pub apply_value_unknown: Option<String>,
    pub apply_value_instance: Option<String>,
    pub skip_apply: Option<String>,
    pub before_apply: Option<String>,
    pub after_apply: Option<String>,
    pub after_apply_from: Option<String>,
    pub after_new_from_doc: Option<String>,
    pub after_update_from_doc: Option<String>,
    pub after_apply_from_doc: Option<String>,
    pub after_new_before_apply: Option<String>,
}

impl From<&LiveHookTrait> for SafeLiveHookTrait {
    fn from(value: &LiveHookTrait) -> Self {
        let LiveHookTrait {
            before_live_design,
            apply_value_unknown,
            apply_value_instance,
            skip_apply,
            before_apply,
            after_apply,
            after_apply_from,
            after_new_from_doc,
            after_update_from_doc,
            after_apply_from_doc,
            after_new_before_apply,
        } = value;

        Self {
            before_live_design: before_live_design.as_ref().map(|x| x.to_string()),
            apply_value_unknown: apply_value_unknown.as_ref().map(|x| x.to_string()),
            apply_value_instance: apply_value_instance.as_ref().map(|x| x.to_string()),
            skip_apply: skip_apply.as_ref().map(|x| x.to_string()),
            before_apply: before_apply.as_ref().map(|x| x.to_string()),
            after_apply: after_apply.as_ref().map(|x| x.to_string()),
            after_apply_from: after_apply_from.as_ref().map(|x| x.to_string()),
            after_new_from_doc: after_new_from_doc.as_ref().map(|x| x.to_string()),
            after_update_from_doc: after_update_from_doc.as_ref().map(|x| x.to_string()),
            after_apply_from_doc: after_apply_from_doc.as_ref().map(|x| x.to_string()),
            after_new_before_apply: after_new_before_apply.as_ref().map(|x| x.to_string()),
        }
    }
}

impl SafeLiveHookTrait {
    pub fn to_token_stream(&self, target: Ident) -> TokenStream{
        fn handle(item: Option<&String>) -> Option<TokenStream>{
            item.map(|x| parse_str::<TokenStream>(x).unwrap())
        }

        let before_live_design = handle(self.before_live_design.as_ref());
        let apply_value_unknown = handle(self.apply_value_unknown.as_ref());
        let apply_value_instance = handle(self.apply_value_instance.as_ref());
        let skip_apply = handle(self.skip_apply.as_ref());
        let before_apply = handle(self.before_apply.as_ref());
        let after_apply = handle(self.after_apply.as_ref());
        let after_apply_from = handle(self.after_apply_from.as_ref());
        let after_new_from_doc = handle(self.after_new_from_doc.as_ref());
        let after_update_from_doc = handle(self.after_update_from_doc.as_ref());
        let after_apply_from_doc = handle(self.after_apply_from_doc.as_ref());
        let after_new_before_apply = handle(self.after_new_before_apply.as_ref());

        quote! {
            impl LiveHook for #target{
                #before_live_design
                #apply_value_unknown
                #apply_value_instance
                #skip_apply
                #before_apply
                #after_apply
                #after_apply_from
                #after_new_from_doc
                #after_update_from_doc
                #after_apply_from_doc
                #after_new_before_apply
            }
        }
    }
}