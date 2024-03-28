use proc_macro2::Span;
use syn::{token::Bracket, AttrStyle, Attribute, Ident, Meta, PathSegment};

#[macro_export]
macro_rules! str_to_string_try_from {
    ($Target:ty) => {
        impl TryFrom<&String> for $Target {
            type Error = Errors;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                value.as_str().try_into()
            }
        }
    };
}


pub fn build_attr_macro(ident:&str)-> Attribute{
    Attribute{
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: Meta::Path(syn::Path::from(PathSegment{
            ident: Ident::new(ident, Span::call_site()),
            arguments: syn::PathArguments::None,
        })),
    }
}