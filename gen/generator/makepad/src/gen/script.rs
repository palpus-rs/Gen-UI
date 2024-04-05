use gen_converter::model::script::LifeTime;
use gen_utils::common::{
    token_stream_to_tree, token_streams_to_trees, token_tree_ident, token_tree_punct_alone,
    tree_to_token_stream, trees_to_token_stream,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Attribute, Lifetime, Meta, Stmt, StmtMacro};

use crate::utils::{
    derive_default_none, derive_live_livehook, handle_shutdown, handle_startup, impl_match_event,
};

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
            let mut derives = derive_live_livehook();

            if is_component {
                derives.extend(vec![
                    token_tree_ident("Widget"),
                    token_tree_punct_alone(','),
                ]);
            }
            // find derive
            change_derives(prop.attrs.as_mut(), derives, "Prop");

            Some(prop.to_token_stream())
        }
    };
}

pub fn event() -> impl FnMut(Option<syn::ItemEnum>) -> Option<TokenStream> {
    return |event| {
        if event.is_none() {
            None
        } else {
            let mut event = event.unwrap();
            // 去除GenUI带有的`#[derive(Event)]`宏更换为Makepad的`#[derive(DefaultNone)]`宏
            let derives = derive_default_none();

            change_derives(event.attrs.as_mut(), derives, "Event");
            // 移除所有的`#[name]`宏
            event.variants.iter_mut().for_each(|variant| {
                variant.attrs.retain(|attr| !attr.path().is_ident("name"));
            });

            Some(event.to_token_stream())
        }
    };
}

pub fn lifetime() -> impl FnMut(Vec<StmtMacro>, &str, bool) -> Option<TokenStream> {
    return |lifetimes, target, is_component| {
        if is_component {
            return None;
        }

        if lifetimes.is_empty() {
            None
        } else {
            //    let lifetimes = lifetimes.unwrap();
            // 目前lifetimes中有两个宏，一个`on_startup!`一个`on_shutdown!`
            // 在Macro中将tokens提取出来放到GenUI提供的LiftTime中即可

            let lifetime_code = lifetimes
                .into_iter()
                .map(|lifetime| {
                    let tokens = token_stream_to_tree(lifetime.mac.tokens);
                    let item = if lifetime.mac.path.is_ident("on_startup") {
                        LifeTime::StartUp(tree_to_token_stream(handle_startup(tokens)))
                    } else if lifetime.mac.path.is_ident("on_shutdown") {
                        LifeTime::ShutDown(tree_to_token_stream(handle_shutdown(tokens)))
                    } else {
                        panic!("Invalid lifetime macro")
                    };
                    item.to_token_stream()
                })
                .collect::<Vec<TokenStream>>();

            let match_event = impl_match_event(
                token_tree_ident(target),
                token_streams_to_trees(lifetime_code),
            );

            Some(trees_to_token_stream(match_event))
        }
    };
}

pub fn other() -> impl FnMut(Vec<Stmt>) -> Option<TokenStream> {
    return |others| {
        if others.is_empty() {
            None
        } else {
            // 直接放到makepad的MatchEvent trait的start_up函数中
            Some(TokenStream::new())
        }
    };
}

fn change_derives(attrs: &mut Vec<Attribute>, mut derives: Vec<TokenTree>, target: &str) {
    for attr in attrs {
        if attr.path().is_ident("derive") {
            if let Meta::List(meta) = &mut attr.meta {
                // remove Prop
                let tmp = meta.tokens.clone();

                tmp.into_iter().for_each(|token| {
                    derives.push(token);
                });

                let pos = derives
                    .iter()
                    .position(|item| item.to_string().eq(target))
                    .unwrap();

                // 向前查找一个逗号，若是逗号则一起删除逗号，否则只删除Prop
                if let Some(TokenTree::Punct(punct)) = derives.get(pos - 1) {
                    if punct.as_char() == ',' {
                        derives.splice(pos - 1..pos + 1, None);
                    }
                } else {
                    derives.remove(pos);
                }

                meta.tokens = TokenStream::from_iter(derives.into_iter());

                break;
            }
        }
    }
}
