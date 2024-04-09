use std::collections::HashMap;

use gen_converter::model::{
    script::{LifeTime, ScriptBuilder, ScriptHandle, ScriptHandles},
    PropTree,
};

use gen_parser::PropsKey;
use gen_utils::common::{
    token_stream_to_tree, token_tree_ident, token_tree_punct_alone, tree_to_token_stream,
    trees_to_token_stream,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{Attribute, Meta, Pat, Stmt, StmtMacro};

use crate::{
    utils::{
        apply_over_and_redraw, derive_default_none, derive_live_livehook, draw_walk,
        handle_actions, handle_event_widget, handle_shutdown, handle_startup, impl_match_event,
        impl_target, instance, instance_new_fn, instance_return_self,
    },
    widget::Widget,
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

pub fn lifetime() -> impl FnMut(Vec<StmtMacro>, bool) -> Option<Vec<LifeTime>> {
    return |lifetimes, is_component| {
        if is_component {
            return None;
        }

        if lifetimes.is_empty() {
            None
        } else {
            // 目前lifetimes中有两个宏，一个`on_startup!`一个`on_shutdown!`
            // 在Macro中将tokens提取出来放到GenUI提供的LiftTime中即可
            // 这两个宏目前只能在Makepad的主组件中使用

            let lifetime_code = lifetimes
                .into_iter()
                .map(|lifetime| {
                    // let tokens = token_stream_to_tree(lifetime.mac.tokens);
                    let tokens = lifetime.mac.tokens;
                    return if lifetime.mac.path.is_ident("on_startup") {
                        // LifeTime::StartUp(tree_to_token_stream(handle_startup(tokens)))
                        LifeTime::StartUp(tokens)
                    } else if lifetime.mac.path.is_ident("on_shutdown") {
                        // LifeTime::ShutDown(tree_to_token_stream(handle_shutdown(tokens)))
                        LifeTime::ShutDown(tokens)
                    } else {
                        panic!("Invalid lifetime macro")
                    };
                })
                .collect::<Vec<LifeTime>>();

            // let match_event = impl_match_event(
            //     token_tree_ident(target),
            //     token_streams_to_trees(lifetime_code),
            // );

            // Some(trees_to_token_stream(match_event))
            Some(lifetime_code)
        }
    };
}

pub fn other() -> impl FnMut(Vec<Stmt>, Option<(PropTree, PropTree)>) -> Option<ScriptHandle> {
    return |others, props| {
        if others.is_empty() {
            None
        } else {
            // 如果是define widget
            // prop绑定部分直接放到makepad的Widget trait的draw_walk函数中,事件部分直接放到handle_event中
            // 如果不是自定义的组件则将所有的others放到makepad的MatchEvent的start_up函数中,事件部分直接放到handle_actions中
            // 不过这里只需要处理内部，真正的函数的包装在scirpt_builder中处理
            if props.is_none() {
                return None;
            }

            let (binds, fns) = props.unwrap();
            let mut is_root = true;
            let mut res = ScriptHandle::default();
            for stmt in others {
                // 获取stmt的标识符
                handle_stmt(&mut res, stmt, &binds, &fns, is_root);
                is_root = false;
            }

            Some(res)
        }
    };
}

fn handle_stmt(
    sc: &mut ScriptHandle,
    stmt: Stmt,
    binds: &PropTree,
    fns: &PropTree,
    root: bool,
) -> () {
    match &stmt {
        Stmt::Local(local) => {
            // 这里将属性绑定和方法绑定分开，方法使用闭包，属性使用变量
            let init = local.init.as_ref().unwrap();
            let expr = &*init.expr;
            match expr {
                syn::Expr::Lit(_) | syn::Expr::Call(_) => {
                    // 属性,获取属性的标识符
                    let ident = get_prop_ident(&local.pat);
                    // 遍历binds
                    if binds.is_empty() {
                        panic!("can not find target bind");
                    } else {
                        let (mut tag, mut id, mut p) = (None, None, None);
                        // println!("{:#?}", binds);
                        for (special, props) in binds {
                            if props.is_none() {
                                continue;
                            } else {
                                if props.as_ref().unwrap().is_empty() {
                                    continue;
                                }

                                let target = props
                                    .as_ref()
                                    .unwrap()
                                    .into_iter()
                                    .find(|(_, v)| ident.eq(v.is_bind_and_get().unwrap()));

                                if let Some((t_p, _)) = target {
                                    tag.replace(special.0.to_string());
                                    id.replace(special.1.to_string());
                                    p.replace(t_p.clone());
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                        sc.push_props(ScriptHandles::Prop(
                            tag.unwrap(),
                            id.unwrap(),
                            p.unwrap(),
                            ident,
                            stmt.to_token_stream(),
                            root,
                        ));
                        // 生成属性绑定的代码
                        // return ScriptHandles::Prop(tag, id, p, stmt.to_token_stream(), root);
                    }
                }
                syn::Expr::Closure(_) => {
                    // 方法
                    let ident = get_prop_ident(&local.pat);
                    if fns.is_empty() {
                        panic!("can not find target fn")
                    } else {
                        let (mut tag, mut id, mut p) = (None, None, None);
                        for (special, events) in fns {
                            if events.is_none() {
                                continue;
                            } else {
                                if events.as_ref().unwrap().is_empty() {
                                    continue;
                                }
                                let target =
                                    events.as_ref().unwrap().into_iter().find(|(_, v)| {
                                        ident.eq(v.is_fn_and_get().unwrap().get_name())
                                    });
                                if let Some((t_p, _)) = target {
                                    tag.replace(special.0.to_string());
                                    id.replace(special.1.to_string());
                                    p.replace(t_p.clone());
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                        // 生成方法绑定的代码
                        sc.push_events(ScriptHandles::Event(
                            tag.unwrap(),
                            id.unwrap(),
                            p.unwrap(),
                            ident,
                            stmt.to_token_stream(),
                            root,
                        ));
                    }
                }
                other => todo!(
                    "can not handle this kind of stmt: `gen::script::handle_stmt` => \n {:#?}",
                    other
                ),
            }
        }
        other => sc.push_others(ScriptHandles::Other(other.to_token_stream())),
    }
}

fn get_prop_ident(pat: &Pat) -> String {
    match pat {
        syn::Pat::Ident(i) => i.ident.to_string(),
        // syn::Pat::Lit(_) => todo!(),
        // syn::Pat::Macro(_) => todo!(),
        // syn::Pat::Or(_) => todo!(),
        // syn::Pat::Paren(_) => todo!(),
        // syn::Pat::Path(_) => todo!(),
        // syn::Pat::Range(_) => todo!(),
        // syn::Pat::Reference(_) => todo!(),
        // syn::Pat::Rest(_) => todo!(),
        // syn::Pat::Slice(_) => todo!(),
        // syn::Pat::Struct(_) => todo!(),
        // syn::Pat::Tuple(_) => todo!(),
        // syn::Pat::TupleStruct(_) => todo!(),
        syn::Pat::Type(t) => get_prop_ident(&*t.pat),
        // syn::Pat::Verbatim(_) => todo!(),
        // syn::Pat::Wild(_) => todo!(),
        _ => panic!("prop bind should use `let` to bind a variable"),
    }
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

#[derive(Debug, Clone)]
pub struct FieldTable {
    prefix: TokenStream,
    fields: Vec<TokenTree>,
}

impl FieldTable {
    pub fn new(prefix: TokenStream, fields: Vec<TokenTree>) -> Self {
        Self { prefix, fields }
    }
    pub fn get_prefix(&self) -> TokenStream {
        self.prefix.clone()
    }
    pub fn get_fields(&self) -> &Vec<TokenTree> {
        self.fields.as_ref()
    }
}

pub fn schandle_to_token_stream<P, E, O>(
    sh: ScriptHandle,
    mut p: P,
    mut e: E,
    mut o: O,
) -> ((TokenStream, TokenStream), TokenStream, TokenStream)
where
    P: FnMut(Vec<ScriptHandles>) -> (FieldTable, TokenStream, TokenStream),
    E: FnMut(Vec<ScriptHandles>, &FieldTable) -> TokenStream,
    O: FnMut(Vec<ScriptHandles>) -> TokenStream,
{
    let ScriptHandle {
        props,
        events,
        others,
    } = sh;
    let (field_table, instance, prop_impl) = p(props);
    ((instance, prop_impl), e(events, &field_table), o(others))
}

/// 在这里uses,props和events无需处理
/// 只需要根据sc_builder中的is_component判断是否是自定义组件还是主组件
/// ## 主：
/// - 将props放到MatchEvent的handle_startup中(也就是直接放到ScriptBuilder的lifetimes的LifeTime::StartUp中)
/// - 将events放到MatchEvent的handle_actions中
/// - 将others放到外部
/// ## 自定义：
/// 自定义组件中无需写lifetime的TokenStream
/// - 将props放到Widget的draw_walk中
/// - 将events放到Widget的handle_event中
/// - 将others放到外部
pub fn sc_builder_to_token_stream(sc_builder: ScriptBuilder) -> TokenStream {
    let ScriptBuilder {
        uses,
        props,
        events,
        lifetimes,
        target,
        is_component,
        others,
    } = sc_builder;
    let impl_target = token_tree_ident(&target);

    let mut t_s = TokenStream::new();
    if let Some(uses) = uses {
        t_s.extend(uses);
    }
    if let Some(props) = props {
        t_s.extend(props);
    }
    if let Some(events) = events {
        t_s.extend(events);
    }
    if let Some(lifetimes) = lifetimes {
        if is_component {
            // 添加Widget的外壳
            // 直接从others中获取prop和event转为tokenstream
            if let Some(sc) = others {
                // let (p_token, e_token, o_token) =
                //     sc.to_token_stream(widget_prop(), widget_event(), widget_other());

                // t_s.extend(p_token.1);
                // t_s.extend(e_token);
                // t_s.extend(o_token);
                todo!()
            }
        } else {
            // 这里需要给生命周期的函数添加Makepad的MatchEvent的外壳
            let mut fn_tks = Vec::new();
            let p_token = if let Some(sc) = others {
                let ((instance, p_token), e_token, o_token) = schandle_to_token_stream(
                    sc,
                    widget_prop_main(),
                    widget_event_main(),
                    widget_other(),
                );
                t_s.extend(instance);
                fn_tks.extend(e_token);
                p_token
                // todo!(
                //     "{}",
                //     format!(
                //         "{:#?}\n{:#?}\n{:#?}",
                //         p_token.0.to_string(),
                //         p_token.1.to_string(),
                //         e_token.to_string()
                //     )
                // );
            } else {
                TokenStream::new()
            };
            let mut start_up_flag = false;
            for lt in lifetimes {
                let fn_tk = if let LifeTime::StartUp(mut start_up) = lt {
                    if start_up_flag {
                        panic!("LifeTime StartUp can only be used once");
                    } else {
                        // p_token add to start_up
                        start_up_flag = true;
                        start_up.extend(p_token.clone());
                        handle_startup(token_stream_to_tree(start_up))
                    }
                } else if let LifeTime::ShutDown(shut_down) = lt {
                    handle_shutdown(token_stream_to_tree(shut_down))
                } else {
                    panic!("Invalid lifetime macro")
                };

                fn_tks.extend(fn_tk);
            }
            let match_event = impl_match_event(token_tree_ident(&target), fn_tks);

            t_s.extend(match_event);
        }
    }

    t_s
}

fn widget_other() -> impl FnMut(Vec<ScriptHandles>) -> TokenStream {
    return |o| TokenStream::from_iter(o.into_iter().map(|item| item.is_other_and_get()));
}
/// 返回
/// - Instance struct（这个可以直接写出去）
/// - handle_startup的内部代码（这个需要进一步加到LifeTime里）
fn widget_prop_main() -> impl FnMut(Vec<ScriptHandles>) -> (FieldTable, TokenStream, TokenStream) {
    return |p| {
        let mut p_map = HashMap::new();
        // 整理到Map中
        p.into_iter().for_each(|item| {
            let (tag, id, prop, ident, code, is_root) = item.is_prop_and_get();

            p_map
                .entry((tag, id))
                .or_insert_with(Vec::new)
                .push((prop, ident, code, is_root))
        });
        let mut tk = TokenStream::new();
        let mut ft_tks = Vec::new();
        let mut init_tks = Vec::new();
        let mut field_tks = Vec::new();
        p_map.into_iter().for_each(|((tag, id), pvs)| {
            let widget = Widget::from(tag.as_str());
            let (ft_tk, init_tk, field_tk, p_tk) = widget.props_from_tk(tag, id, pvs);
            tk.extend(p_tk);
            ft_tks.extend(ft_tk);
            init_tks.extend(init_tk);
            field_tks.extend(field_tk);
        });
        // build Instance and back a field table
        (
            FieldTable::new(
                trees_to_token_stream(vec![
                    token_tree_ident("instance"),
                    token_tree_punct_alone('.'),
                ]),
                field_tks.clone(),
            ),
            trees_to_token_stream(build_instance(ft_tks, init_tks, field_tks)),
            tk,
        )
    };
}

fn build_instance(
    tk: Vec<TokenTree>,
    init_tks: Vec<TokenTree>,
    field_tks: Vec<TokenTree>,
) -> Vec<TokenTree> {
    let mut tks = instance(tk);
    tks.extend(impl_target(
        "Instance",
        instance_new_fn(instance_return_self(init_tks, field_tks)),
    ));
    tks
}

/// 构建handle_actions
fn widget_event_main() -> impl FnMut(Vec<ScriptHandles>, &FieldTable) -> TokenStream {
    return |e, field_table| {
        // 事件和属性不同，都是单条的，即使是同一个组件也是单条处理的

        let mut tks = Vec::new();
        e.into_iter().for_each(|item| {
            let (tag, id, event, ident, code, is_root) = item.is_event_and_get();
            let widget = Widget::from(tag.as_str());
            // let tk = if is_root {
            //     widget.events(Some(id.clone()), id, (event, ident, code), field_table)
            // } else {
            //     widget.events(None, id, (event, ident, code), field_table)
            // };
            let tk = widget.events(Some(id.clone()), id, (event, ident, code), field_table);
            tks.extend(tk);
        });
        trees_to_token_stream(handle_actions(tks))
    };
}

/// 在Widget trait中添加draw_walk函数
fn widget_prop() -> impl FnMut(Vec<ScriptHandles>) -> TokenStream {
    return |p| {
        dbg!(&p);

        let mut tks = TokenStream::new();
        let w_p = p.into_iter().for_each(|item| {
            let (tag, id, prop, ident, code, is_root) = item.is_prop_and_get();
            let widget = Widget::from(tag.as_str());
        });

        tks
    };
}

fn widget_event() -> impl FnMut(Vec<ScriptHandles>) -> TokenStream {
    return |e| {
        dbg!(e);
        todo!()
        //    TokenStream::from_iter(handle_event_widget())
    };
}
