use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_derive_event(derive_input: &DeriveInput) -> TokenStream {
    let enum_name = &derive_input.ident;
    let expanded = quote! {
        impl Event for #enum_name{
            
        }
    };

    TokenStream::from(expanded)
}
