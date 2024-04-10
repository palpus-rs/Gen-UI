use std::collections::HashMap;

use gen_converter::model::script::{LifeTime, ScriptBuilder, ScriptHandle, ScriptHandles};

use gen_utils::common::{
    camel_to_snake, token_stream_to_tree, token_tree_group, token_tree_group_bracket,
    token_tree_ident, token_tree_punct_alone, token_tree_punct_joint, trees_to_token_stream,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse2;

use crate::{
    utils::{
        derive_live_livehook, derive_macros, event_start_up, handle_actions, handle_event,
        handle_shutdown, handle_startup, impl_app_main, impl_live_register, impl_match_event,
        impl_target, instance, instance_new, instance_new_fn, instance_return_self, live_attr,
        macro_app_main, makepad_widgets_register, match_event, match_item, match_other, rust_attr,
        self_handle_event, self_handle_startup, self_match_event, struct_field_type,
    },
    widget::Widget,
};

mod other;
pub use other::*;

#[derive(Debug, Clone)]
pub struct FieldItem {
   pub source: Widget,
   pub prop: String,
   pub value: String,
}

impl FieldItem {
    pub fn to_field_tk(&self)->Vec<TokenTree>{
        vec![
            token_tree_ident(&self.value),
            token_tree_punct_alone(','),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct FieldTable {
    prefix: TokenStream,
    fields: Vec<FieldItem>,
}

impl FieldTable {
    pub fn new(prefix: TokenStream, fields: Vec<FieldItem>) -> Self {
        Self { prefix, fields }
    }
    pub fn get_prefix(&self) -> TokenStream {
        self.prefix.clone()
    }
    pub fn get_fields(&self) -> &Vec<FieldItem> {
        self.fields.as_ref()
    }
    /// add `self.` to prefix
    pub fn self_prefix(&self) -> TokenStream {
        let mut tk = TokenStream::new();
        tk.extend(vec![token_tree_ident("self"), token_tree_punct_alone('.')]);
        tk.extend(self.prefix.clone());
        tk
    }
    pub fn to_field_strs(&self) -> Vec<String> {
        self.fields.iter().map(|item| item.value.clone()).collect()
    }
    
}

pub fn schandle_to_token_stream<P, E, O>(
    sh: ScriptHandle,
    root: Option<String>,
    mut p: P,
    mut e: E,
    mut o: O,
) -> ((TokenStream, TokenStream), TokenStream, TokenStream)
where
    P: FnMut(Vec<ScriptHandles>, Option<String>) -> (FieldTable, TokenStream, TokenStream),
    E: FnMut(Vec<ScriptHandles>, Option<String>, &FieldTable) -> TokenStream,
    O: FnMut(Vec<ScriptHandles>) -> TokenStream,
{
    let ScriptHandle {
        props,
        events,
        others,
    } = sh;
    let (field_table, instance, prop_impl) = p(props, root.clone());
    (
        (instance, prop_impl),
        e(events, root, &field_table),
        o(others),
    )
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
        root,
    } = sc_builder;
    let impl_target = token_tree_ident(&target);

    let mut t_s = TokenStream::new();
    if let Some(uses) = uses {
        // 将GenUI的use去除
        let uses = format!("{{ {} }}", uses).parse::<TokenStream>().unwrap();

        let uses_block = parse2::<syn::Block>(uses).unwrap();

        let uses = uses_block
            .stmts
            .into_iter()
            .filter(|stmt| !stmt.to_token_stream().to_string().contains("gen"))
            .for_each(|item| {
                t_s.extend(item.to_token_stream());
            });
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
                    root.clone(),
                    widget_prop_main(),
                    widget_event_main(),
                    widget_other(),
                );
                t_s.extend(instance);
                fn_tks.extend(e_token);
                p_token
            } else {
                TokenStream::new()
            };
            let mut start_up_flag = false;
            let mut shut_down_flag = false;
            for lt in lifetimes {
                let fn_tk = if let LifeTime::StartUp(start_up) = lt {
                    if start_up_flag {
                        panic!("LifeTime StartUp can only be used once");
                    } else {
                        // p_token add to start_up
                        start_up_flag = true;
                        let mut start_up_tk = TokenStream::new();
                        start_up_tk.extend(instance_new());
                        start_up_tk.extend(p_token.clone());
                        start_up_tk.extend(start_up);

                        handle_startup(token_stream_to_tree(start_up_tk))
                    }
                } else if let LifeTime::ShutDown(shut_down) = lt {
                    handle_shutdown(token_stream_to_tree(shut_down))
                } else {
                    panic!("Invalid lifetime macro")
                };

                fn_tks.extend(fn_tk);
            }
            // build root struct
            t_s.extend(build_root_struct(&target, root.as_ref().unwrap()));
            let match_event_wrap = impl_match_event(token_tree_ident(&target), fn_tks);

            t_s.extend(match_event_wrap);
            // impl AppMain
            t_s.extend(impl_app_main(
                &target,
                handle_event(impl_app_main_tk(root.clone(), start_up_flag)),
            ));

            // 添加LiveRegister
            // 暂时只添加makepad_widgets的live_register
            // 这个需要根据组件注册的情况来添加
            t_s.extend(impl_live_register(
                &target,
                makepad_widgets_register("makepad_widgets"),
            ));
            // 添加app_main!
            t_s.extend(macro_app_main(&target));
        }
    }

    t_s
}

pub fn build_root_struct(target: &str, root: &str) -> Vec<TokenTree> {
    let mut tk = derive_macros(vec!["Live", "LiveHook"]);
    tk.extend(vec![
        token_tree_ident("pub"),
        token_tree_ident("struct"),
        token_tree_ident(target),
    ]);

    let mut fields = Vec::new();
    fields.extend(live_attr());
    fields.extend(struct_field_type(root, token_tree_ident("WidgetRef")));
    fields.extend(rust_attr());
    fields.extend(struct_field_type("instance", token_tree_ident("Instance")));
    tk.push(token_tree_group(fields));
    tk
}

/// 实现完整的AppMain的TokenStream
fn impl_app_main_tk(target: Option<String>, start_up_flag: bool) -> Vec<TokenTree> {
    // 构建match event中的事件处理
    let mut handle_event_code = Vec::new();
    if start_up_flag {
        handle_event_code.extend(match_item(event_start_up(), self_handle_startup()));
    }
    handle_event_code.extend(match_other());
    let mut inner_code = match_event(handle_event_code);
    inner_code.extend(self_match_event());
    inner_code.extend(self_handle_event(target));

    inner_code
}

fn widget_other() -> impl FnMut(Vec<ScriptHandles>) -> TokenStream {
    return |o| TokenStream::from_iter(o.into_iter().map(|item| item.is_other_and_get()));
}
/// 返回
/// - Instance struct（这个可以直接写出去）
/// - handle_startup的内部代码（这个需要进一步加到LifeTime里）
fn widget_prop_main(
) -> impl FnMut(Vec<ScriptHandles>, Option<String>) -> (FieldTable, TokenStream, TokenStream) {
    return |p, root| {
        let mut p_map = HashMap::new();
        // 整理到Map中
        p.into_iter().for_each(|item| {
            let (tag, id, prop, ident, code, is_root) = item.is_prop_and_get();

            p_map
                .entry((tag, id))
                .or_insert_with(Vec::new)
                .push((prop, ident, code, is_root))
        });
        // 收集所有需要的Token
        let mut tk = TokenStream::new();
        let mut ft_tks = Vec::new();
        let mut init_tks = Vec::new();
        let mut field_items = Vec::new();
        p_map.into_iter().for_each(|((tag, id), pvs)| {
            let widget = Widget::from(tag.as_str());
            let (ft_tk, init_tk, field_tk, p_tk) = widget.props_from_tk(root.clone(), tag, id, pvs);
            tk.extend(p_tk);
            ft_tks.extend(ft_tk);
            init_tks.extend(init_tk);
            field_items.extend(field_tk);
        });
        // build Instance and back a field table

        let mut field_tks =Vec::new();
         field_items.iter().for_each(|item| field_tks.extend(item.to_field_tk()));

        (
            FieldTable::new(
                trees_to_token_stream(vec![
                    token_tree_ident("instance"),
                    token_tree_punct_alone('.'),
                ]),
                field_items,
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
fn widget_event_main() -> impl FnMut(Vec<ScriptHandles>, Option<String>, &FieldTable) -> TokenStream
{
    return |e, root, field_table| {
        // 事件和属性不同，都是单条的，即使是同一个组件也是单条处理的

        let mut tks = Vec::new();
        e.into_iter().for_each(|item| {
            let (tag, id, event, ident, code, _) = item.is_event_and_get();
            let widget = Widget::from(tag.as_str());
            // let tk = if is_root {
            //     widget.events(Some(id.clone()), id, (event, ident, code), field_table)
            // } else {
            //     widget.events(None, id, (event, ident, code), field_table)
            // };
            let tk = widget.events(root.clone(), id, (event, ident, code), field_table);
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
