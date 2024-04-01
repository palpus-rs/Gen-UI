use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_derive_event(derive_input: &DeriveInput) -> TokenStream {
    let enum_name = &derive_input.ident;
    let expanded = quote! {
        impl Event for #enum_name{
            fn clone_box(&self) -> Box<dyn Event> {
                Box::new(self.clone())
            }
        }
    };

    TokenStream::from(expanded)
}
