use gen_converter::{error::Errors, model::script::PropFn};
use gen_parser::{Function, Value};
use gen_utils::common::{
    token_stream_to_tree, token_tree_group, token_tree_group_paren, token_tree_ident,
    token_tree_punct_alone, trees_to_token_stream,
};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse_quote, visit_mut::VisitMut, Attribute, ItemStruct, Meta, Pat, Stmt};

use crate::{prop::builtin::MakepadValue, utils::apply_over_and_redraw};

use super::BuiltIn;

pub fn bool_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(bool) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<bool>() {
            Ok(b) => {
                f(b);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else {
        value
            .is_bool_and_get()
            .map(|b| {
                f(b);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn f64_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f64) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f64>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else if let Some(d) = value.is_double_and_get() {
        f(d);
        Ok(())
    } else {
        value
            .is_float_and_get()
            .map(|b| {
                f(b as f64);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn f32_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(f32) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        match s.parse::<f32>() {
            Ok(d) => {
                f(d);
                Ok(())
            }
            Err(_) => Err(Errors::PropConvertFail(format!(
                "{} can not convert to show_bg",
                s
            ))),
        }
    } else {
        value
            .is_float_and_get()
            .map(|b| {
                f(b);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn string_prop<F>(value: &Value, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&str) -> (),
{
    if let Some(s) = value.is_unknown_and_get() {
        f(s);
        Ok(())
    } else {
        value
            .is_string_and_get()
            .map(|s| {
                f(s);
                Ok(())
            })
            .unwrap_or_else(|| {
                Err(Errors::PropConvertFail(format!(
                    "{} can not convert to show_bg",
                    value
                )))
            })
    }
}

pub fn bind_prop_value(value: &Value, is_prop: bool, ident: &str) -> String {
    return if let Some(v) = value.is_bind_and_get() {
        if is_prop && ident.starts_with(ident) {
            // 说明这个绑定的属性是从外部传入的，需要将定义的首个prefix转为self
            v.replacen(ident, "self", 1)
        } else {
            v.to_string()
        }
    } else {
        panic!("prop value is not bind")
    };
}

pub fn quote_prop(keys: Vec<&str>, value: &str) -> TokenStream {
    let mut result = String::new();

    // 迭代keys，逐一构建字符串
    for &key in keys.iter() {
        if !result.is_empty() {
            result.push_str(": {");
        }
        result.push_str(key);
    }

    // 添加最内层的值
    result.push_str(" : (");
    result.push_str(value);
    result.push_str("),");

    // 根据keys的数量，添加相应数量的闭括号
    for _ in 0..keys.len() - 1 {
        result.push_str("},");
    }

    result.parse().unwrap()
}

/// 将GenUI的结构体转为Makepad的属性结构体
pub fn quote_makepad_widget_struct(value: &ItemStruct) -> ItemStruct {
    let mut new_item = value.clone();

    // 遍历属性并修改,将Prop修改为Live, LiveHook, Widget
    for attr in new_item.attrs.iter_mut() {
        if let Meta::List(meta) = &mut attr.meta {
            if meta.path.is_ident("derive") && meta.tokens.to_string().contains("Prop") {
                // 使用parse_quote! 宏来创建新的tokens
                meta.tokens = parse_quote! { Live, LiveHook, Widget };
                // 将修改后的Meta赋值回Attribute
                *attr = Attribute {
                    meta: Meta::List(meta.clone()),
                    ..attr.clone()
                }
            }
        }
    }
    // 对结构体中的字段进行处理，符合的进行宏标记
    for field in new_item.fields.iter_mut() {
        let ident = field.ty.to_token_stream().to_string();
        match MakepadValue::from(&ident) {
            MakepadValue::Live(_) => {
                field.attrs.push(parse_quote! { #[live] });
            }
            MakepadValue::Rust => {
                field.attrs.push(parse_quote! { #[rust] });
            }
            _ => panic!("prop ptr field not support to convert to MakepadValue"),
        }
    }
    new_item
}

/// 根据widget的绘制函数生成对应的代码
/// 生成对应widget的绘制函数中的代码
/// 这部分很统一，所有的widget都是这样处理的
pub fn quote_draw_walk(draw_walk: &Option<Vec<PropFn>>) -> Option<TokenStream> {
    let tk = if let Some(draw_walk_tk) = draw_walk {
        let mut tk = TokenStream::new();
        for item in draw_walk_tk {
            let PropFn {
                widget,
                id,
                key,
                ident,
                code,
                is_prop,
            } = item;
            // from widget get prop value
            // 当前只考虑builtin，自定义类型组件后续增加
            let builtin = BuiltIn::from(&widget);
            let pv = builtin.prop_bind(key, ident, *is_prop, &local_ident(code));
            if !is_prop {
                tk.extend(code.to_token_stream());
            }
            tk.extend(apply_over_and_redraw(
                None,
                widget,
                id,
                token_stream_to_tree(pv),
            ));
        }
        Some(tk)
    } else {
        None
    };
    tk
}

/// get local ident from stmt
fn local_ident(code: &Stmt) -> String {
    fn get(pat: &Pat) -> String {
        match pat {
            Pat::Ident(ident) => ident.ident.to_string(),
            Pat::Type(ty) => get(&*ty.pat),
            _ => panic!("local stmt must be ident|type"),
        }
    }

    if let Stmt::Local(local) = code {
        get(&local.pat)
    } else {
        panic!("local stmt must be ident|type")
    }
}

/// 根据widget的事件处理函数生成对应的代码
/// 生成出对应widget的事件处理函数
pub fn quote_handle_event(event: &Option<Vec<PropFn>>, target: Option<TokenTree>) -> TokenStream {
    let tk = if let Some(event_tk) = event {
        let mut tk = TokenStream::new();
        for item in event_tk {
            let PropFn {
                widget,
                id,
                key,
                ident,
                code,
                is_prop,
            } = item;

            let fn_ident = ident.is_fn_and_get().unwrap().to_token_easy();

            // check active! macro and change to makepad cx.widget_action
            let mut code = code.clone();
            active_macro_to_cx_widget_action(&mut code);
            let mut code_tk = code.to_token_stream();
            code_tk.extend(token_stream_to_tree(fn_ident));

            let stmt = vec![
                token_tree_ident("if"),
                token_tree_ident("self"),
                token_tree_punct_alone('.'),
                token_tree_ident(widget),
                token_tree_group_paren(vec![
                    token_tree_ident("id"),
                    token_tree_punct_alone('!'),
                    token_tree_group_paren(vec![token_tree_ident(id)]),
                ]),
                token_tree_punct_alone('.'),
                token_tree_ident(key.name()),
                token_tree_group_paren(vec![token_tree_ident("actions")]),
                token_tree_group(token_stream_to_tree(code_tk)),
            ];

            tk.extend(stmt);
        }
        Some(tk)
    } else {
        None
    };

    let target_handle_tk = match target {
        Some(t) => Some(quote! {self.#t.handle_event(cx, event, scope);}),
        None => None,
    };

    quote! {
        let uid = self.widget_uid();
        if let Event::Actions(actions) = event{
            #tk
        }
        #target_handle_tk
    }
}

pub fn active_macro_to_cx_widget_action(code: &mut Stmt) -> TokenStream {
    struct MacroModifier;
    impl VisitMut for MacroModifier {
        fn visit_expr_block_mut(&mut self, i: &mut syn::ExprBlock) {
            for stmt in i.block.stmts.iter_mut() {
                if let Stmt::Macro(macro_stmt) = stmt {
                    if macro_stmt.mac.path.is_ident("active") {
                        let tk = &macro_stmt.mac.tokens;
                        *stmt = parse_quote! {
                            cx.widget_action(uid, &scope.path, #tk);
                        };
                    }
                }
            }
        }
    }

    MacroModifier.visit_stmt_mut(code);
    code.to_token_stream()
}

#[cfg(test)]
mod test_utils {
    #[test]
    fn test_quote_prop() {
        let keys = vec!["a", "b", "c"];
        let value = "1";
        let result = super::quote_prop(keys, value);
        assert_eq!(result.to_string(), "a : { b : { c : (1) , } , } ,");
    }
}
