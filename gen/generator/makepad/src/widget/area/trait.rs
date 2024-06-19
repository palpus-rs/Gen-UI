use gen_converter::model::script::PropFn;

use gen_utils::common::ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::widget::utils::quote_handle_event;

pub fn handle_event(
    event: &Option<Vec<PropFn>>,
    props: &Option<Vec<PropFn>>,
    instance_name: Option<&Ident>,
    prop_fields: Option<&Vec<Ident>>,
) -> TokenStream {
    quote_handle_event(
        Some(ident("view")),
        event,
        props,
        instance_name,
        prop_fields,
    )
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
