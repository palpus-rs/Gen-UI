use gen_converter::model::script::PropFn;

use gen_utils::common::token_tree_ident;
use proc_macro2::TokenStream;
use quote::quote;

use crate::widget::utils::quote_handle_event;

pub fn handle_event(event: &Option<Vec<PropFn>>) -> TokenStream {
    quote_handle_event(event, Some(token_tree_ident("view")))
}

pub fn draw_walk(draw_walk: &Option<TokenStream>) -> TokenStream {
    // let tk = quote_draw_walk(draw_walk);

    quote! {
        cx.begin_turtle(walk, self.layout);
        #draw_walk
        cx.end_turtle();
        DrawStep::done()
    }
}
