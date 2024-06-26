use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

mod event;

use event::impl_derive_event;

#[proc_macro_derive(Event, attributes(name))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_derive_event(&input)
}

#[proc_macro_attribute]
pub fn name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// #[proc_macro_derive(Prop)]
// pub fn derive_prop(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;
//     let expanded = quote! {
//         impl Prop for #name{
            
//         }
//     };

//     TokenStream::from(expanded)
    
// }

#[proc_macro_derive(Prop)]
pub fn derive_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _name = &input.ident;
    // let expanded = quote! {
    //     impl Prop for #name{
            
    //     }
    // };

    let Data::Struct(_prop_struct) = &input.data else {
        panic!("Prop derive only works on structs");
    };

    // prop_struct.fields.iter()
   

    TokenStream::from(quote!(#input))
    
}