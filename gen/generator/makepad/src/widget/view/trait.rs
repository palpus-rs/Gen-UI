use gen_converter::model::script::PropFn;

use proc_macro2::TokenStream;
use quote::quote;

use crate::widget::utils::{quote_draw_walk, quote_handle_event};

pub fn handle_event(event: &Option<Vec<PropFn>>) -> TokenStream {
    quote_handle_event(event, None)
}

pub fn draw_walk(draw_walk: &Option<Vec<PropFn>>) -> TokenStream {
    let tk = quote_draw_walk(draw_walk);

    quote! {
        
        #tk
        self.view.draw_walk(cx, scope, walk)
    }
}
