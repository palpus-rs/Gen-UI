use proc_macro2::{TokenStream, TokenTree};
use syn::{Block, Meta, Stmt, StmtMacro};

use crate::{error::Errors, model::Model};

/// 在GenUI中Rust脚本是直接写在`<script>`标签里的
/// 例如：`<script>println!("Hello, World!");</script>`
/// 其实最主要的目的在于为TemplateModel的prop_ptr和event_ptr赋值
/// 这个策略器中包含有多个闭包
/// - use 策略器： 处理use语句，获取所有use语句并返回TokenTree
/// - prop 策略器：获取带有`#[derive(Prop)]`的结构体，并返回TokenTree
/// - event 策略器：获取带有`#[derive(Event)]`的枚举，并返回TokenTree
/// - 生命周期策略器: 处理生命周期,目前只处理带有`on_startup!, on_shutdown!`标识的返回TokenTree
/// - 其他策略器： 用于处理其他的语句，例如`let a = 1;`等
pub fn script<U, P, E, L, F>(
    model: Model,
    mut use_f: U,
    mut prop_f: P,
    mut event_f: E,
    mut lifetime_f: L,
    mut other_f: F,
) -> Result<TokenStream, Errors>
where
    U: FnMut(Vec<syn::ItemUse>) -> Option<TokenStream>,
    P: FnMut(Option<syn::ItemStruct>) -> Option<TokenStream>,
    E: FnMut(Option<syn::ItemEnum>) -> Option<TokenStream>,
    L: FnMut(Vec<StmtMacro>) -> Option<TokenStream>,
    F: FnMut(Vec<Stmt>) -> Option<TokenStream>,
{
    if !model.has_script() {
        return Err(Errors::StrategyNoScript);
    }
    let mut tt = TokenStream::new();
    let script = model.script.unwrap().to_origin();
    let (uses, prop, event, lifetime, other) = split_script(script);

    extend(&mut tt, use_f(uses));
    extend(&mut tt, prop_f(prop));
    extend(&mut tt, event_f(event));
    extend(&mut tt, lifetime_f(lifetime));
    extend(&mut tt, other_f(other));

    Ok(tt)
}

fn extend(iter: &mut TokenStream, ts: Option<TokenStream>) -> () {
    if let Some(value) = ts {
        iter.extend(value);
    }
}

fn split_script(
    block: Block,
) -> (
    Vec<syn::ItemUse>,
    Option<syn::ItemStruct>,
    Option<syn::ItemEnum>,
    Vec<StmtMacro>,
    Vec<syn::Stmt>,
) {
    let stmts = block.stmts;
    let mut uses = Vec::new();
    let mut prop = None;
    let mut event = None;
    let mut other = Vec::new();
    let mut lifetime = Vec::new();

    for stmt in &stmts {
        match stmt {
            syn::Stmt::Item(item) => {
                match item {
                    syn::Item::Use(use_item) => uses.push(use_item.clone()),
                    syn::Item::Struct(struct_item) => {
                        // 查看是否有`#[derive(Prop)]`的属性
                        // 如果有则将其将prop设置为Some
                        // 否则放到other中
                        for attr in struct_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Prop")
                                {
                                    if prop.is_none() {
                                        prop.replace(struct_item.clone());
                                    } else {
                                        panic!("Only one struct can be derived from Prop")
                                    }
                                } else {
                                    other.push(stmt.clone());
                                }
                            }
                        }
                    }
                    syn::Item::Enum(enum_item) => {
                        // 处理带有`#[derive(Event)]`的枚举
                        // 如果有则将其将event设置为Some
                        // 否则放到other中
                        for attr in enum_item.clone().attrs {
                            if let Meta::List(list) = &attr.meta {
                                if list.path.is_ident("derive")
                                    && list.tokens.to_string().contains("Event")
                                {
                                    if event.is_none() {
                                        event.replace(enum_item.clone());
                                    } else {
                                        panic!("Only one enum can be derived from Event")
                                    }
                                } else {
                                    other.push(stmt.clone());
                                }
                            }
                        }
                    }
                    _ => {
                        // 其他情况也直接放到other中
                        other.push(stmt.clone());
                    }
                }
            }
            syn::Stmt::Macro(item) => {
                // 处理生命周期
                // 目前只处理带有`on_startup!, on_shutdown!`标识的
                // 其他的放到other中
                if item.mac.path.is_ident("on_startup") || item.mac.path.is_ident("on_shutdown") {
                    // 处理生命周期
                    lifetime.push(item.clone());
                } else {
                    other.push(stmt.clone());
                }
            }
            _ => {
                // 其他情况直接放到other中
                other.push(stmt.clone());
            }
        }
    }
    (uses, prop, event, lifetime, other)
}
