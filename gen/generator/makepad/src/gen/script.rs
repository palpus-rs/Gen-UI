use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{parse2, Stmt, StmtMacro};

pub fn r#use() -> impl FnMut(Vec<syn::ItemUse>) -> Option<TokenStream> {
    return |uses| {
        if uses.is_empty(){
            None
        }else {
            Some(uses.into_iter()
            .map(|use_item| {
                use_item
                    .to_token_stream()
                    .into_iter()
                    .collect::<Vec<TokenTree>>()
            })
            .flatten()
            .collect())
        }
    };
}

pub fn prop() -> impl FnMut(Option<syn::ItemStruct>) -> Option<TokenStream> {
    return |prop| {
        if prop.is_none(){
            None
        }else {
           let prop = prop.unwrap();
           Some(TokenStream::new())
        }
    };
}

pub fn event() -> impl FnMut(Option<syn::ItemEnum>) -> Option<TokenStream> {
    return |event| {
        if event.is_none(){
            None
        }else {
           let event = event.unwrap();
           Some(TokenStream::new())
        }
    };
}

pub fn lifetime() -> impl FnMut(Vec<StmtMacro>) -> Option<TokenStream> {
    return |lifetimes| {
        if lifetimes.is_empty(){
            None
        }else {
        //    let lifetimes = lifetimes.unwrap();
           Some(TokenStream::new())
        }
    };
}

pub fn other() -> impl FnMut(Vec<Stmt>) -> Option<TokenStream> {
    return |others| {
        if others.is_empty(){
            None
        }else {
        //    let others = others.unwrap();
           Some(TokenStream::new())
        }
    };
}
