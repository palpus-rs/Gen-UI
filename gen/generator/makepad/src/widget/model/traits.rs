use gen_utils::common::token_tree_ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// 对于Widget来说
/// draw_walk是必须实现的
/// 其他的方法是可选的
#[derive(Debug, Default, Clone)]
pub struct WidgetTrait {
    /// 必须实现
    pub draw_walk: TokenStream,
    /// 可选实现
    pub handle_event: Option<TokenStream>,
    pub widget: Option<TokenStream>,
    pub widgets: Option<TokenStream>,
    pub widget_id: Option<TokenStream>,
    pub widget_to_data: Option<TokenStream>,
    pub data_to_widget: Option<TokenStream>,
    pub draw: Option<TokenStream>,
    pub draw_walk_all: Option<TokenStream>,
    pub is_visible: Option<TokenStream>,
    pub draw_all: Option<TokenStream>,
    pub text: Option<TokenStream>,
    pub set_text: Option<TokenStream>,
    pub set_text_and_redraw: Option<TokenStream>,
    pub ref_cast_type_id: Option<TokenStream>,
}

impl WidgetTrait {
    pub fn new(draw_walk: TokenStream) -> Self {
        let mut widget_trait = WidgetTrait::default();
        widget_trait.draw_walk = draw_walk;
        widget_trait
    }
    pub fn draw_walk(&mut self, tk: TokenStream) -> () {
        self.draw_walk = quote! {
            fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
                #tk
            }
        }
        // self.draw_walk = value.into_iter().map(|item| item.to_token_stream()).collect();
    }
    pub fn handle_event(&mut self, tk: TokenStream) -> () {
        self.handle_event = Some(quote! {
            fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
                #tk
            }
        });
    }
    pub fn to_token_stream(&self, target: Ident) -> TokenStream {
        let draw_walk = &self.draw_walk;
        let handle_event = &self.handle_event;
        let widget = &self.widget;
        let widgets = &self.widgets;
        let widget_id = &self.widget_id;
        let widget_to_data = &self.widget_to_data;
        let data_to_widget = &self.data_to_widget;
        let draw = &self.draw;
        let draw_walk_all = &self.draw_walk_all;
        let is_visible = &self.is_visible;
        let draw_all = &self.draw_all;
        let text = &self.text;
        let set_text = &self.set_text;
        let set_text_and_redraw = &self.set_text_and_redraw;
        let ref_cast_type_id = &self.ref_cast_type_id;

        quote! {
            impl Widget for #target{
                #draw_walk
                #handle_event
                #widget
                #widgets
                #widget_id
                #widget_to_data
                #data_to_widget
                #draw
                #draw_walk_all
                #is_visible
                #draw_all
                #text
                #set_text
                #set_text_and_redraw
                #ref_cast_type_id
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppMainTrait {
    pub handle_event: Option<TokenStream>,
}

impl AppMainTrait {
    pub fn handle_event(&mut self, event: TokenStream) -> () {
        self.handle_event.replace(event);
    }
    pub fn to_token_stream(&self, ui_root: &str) -> TokenStream {
        let tk = self.handle_event.as_ref();
        let ui_root = token_tree_ident(ui_root);
        quote! {
            fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
                #tk
                self.match_event(cx, event);
                self.#ui_root.handle_event(cx, event, &mut Scope::empty());
            }
        }
    }
}
