use gen_converter::model::script::PropFn;

use proc_macro2::TokenStream;
use quote::quote;

use crate::widget::utils::quote_handle_event;

pub fn handle_event(event: &Option<Vec<PropFn>>) -> TokenStream {
    quote_handle_event(event, None)
}

pub fn draw_walk() -> TokenStream {
    quote! {
        self.view.draw_walk(cx, scope, walk)
    }
}
