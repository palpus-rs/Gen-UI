use gen_utils::common::{token_tree_ident, token_tree_punct_alone};
use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse2, token::Token, Meta, Stmt, StmtMacro};

pub fn r#use() -> impl FnMut(Vec<syn::ItemUse>) -> Option<TokenStream> {
    return |uses| {
        if uses.is_empty() {
            None
        } else {
            Some(
                uses.into_iter()
                    .map(|use_item| {
                        use_item
                            .to_token_stream()
                            .into_iter()
                            .collect::<Vec<TokenTree>>()
                    })
                    .flatten()
                    .collect(),
            )
        }
    };
}

pub fn prop() -> impl FnMut(Option<syn::ItemStruct>, bool) -> Option<TokenStream> {
    return |prop, is_component| {
        if prop.is_none() {
            None
        } else {
            let mut prop = prop.unwrap();
            // 去除GenUI带有的`#[derive(Prop)]`宏更换为Makepad的宏
            // 若不是自定义组件则使用#[derive(Live, LiveHook)]
            // 若是自定义组件则使用#[derive(Live, LiveHook, Widget)]
            let mut derives = vec![
                token_tree_ident("Live"),
                token_tree_punct_alone(','),
                token_tree_ident("LiveHook"),
                token_tree_punct_alone(','),
            ];

            if is_component {
                derives.extend(vec![
                    token_tree_ident("Widget"),
                    token_tree_punct_alone(','),
                ]);
            }
            // find derive
            for attr in &mut prop.attrs {
                if attr.path().is_ident("derive") {
                    if let Meta::List(meta) = &mut attr.meta {
                        // remove Prop
                        let tmp = meta.tokens.clone();
                        
                        tmp.into_iter().for_each(|token| {
                            derives.push(token);
                        });
                       
                        let pos = derives
                            .iter()
                            .position(|item| item.to_string().eq("Prop"))
                            .unwrap();
                        
                        derives.remove(pos);
                        // derives.splice(pos..pos + 2, None);
                        
                        meta.tokens = TokenStream::from_iter(derives.into_iter());

                        break;
                    }
                }
            }
            
            Some(prop.to_token_stream())
        }
    };
}

pub fn event() -> impl FnMut(Option<syn::ItemEnum>) -> Option<TokenStream> {
    return |event| {
        if event.is_none() {
            None
        } else {
            let event = event.unwrap();
            Some(TokenStream::new())
        }
    };
}

pub fn lifetime() -> impl FnMut(Vec<StmtMacro>) -> Option<TokenStream> {
    return |lifetimes| {
        if lifetimes.is_empty() {
            None
        } else {
            //    let lifetimes = lifetimes.unwrap();
            Some(TokenStream::new())
        }
    };
}

pub fn other() -> impl FnMut(Vec<Stmt>) -> Option<TokenStream> {
    return |others| {
        if others.is_empty() {
            None
        } else {
            //    let others = others.unwrap();
            Some(TokenStream::new())
        }
    };
}
