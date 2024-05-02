use gen_converter::model::script::UseMod;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{parse_quote, Attribute, Ident, ItemEnum, ItemStruct, Meta};

use crate::widget::BuiltIn;

pub struct WidgetHandler;

impl WidgetHandler {
    pub fn uses(uses: &UseMod) -> Option<TokenStream> {
        let mut tk = TokenStream::new();
        let UseMod { widget, other, .. } = uses;

        if let Some(widget) = widget {
            for item in widget.iter() {
                tk.extend(item.to_token_stream());
            }
        }

        if let Some(other) = other {
            for item in other.iter() {
                tk.extend(item.to_token_stream());
            }
        }

        if tk.is_empty() {
            None
        } else {
            Some(tk)
        }
    }
    pub fn prop_ptr(prop_ptr: &ItemStruct, inherit: &BuiltIn) -> TokenStream {
        // 将GenUI的结构体转为Makepad的属性结构体
        inherit.to_token_stream(prop_ptr)
    }
    pub fn event_ptr(event_ptr: &ItemEnum) -> TokenStream {
        // 将GenUI的结构体转为Makepad的事件枚举
        // 将GenUI标记的#[derive(Event)]修改为Makepad的#[derive(DefaultNone)]
        let mut new_item = event_ptr.clone();
        for attr in new_item.attrs.iter_mut() {
            if let Meta::List(meta) = &mut attr.meta {
                if meta.path.is_ident("derive") && meta.tokens.to_string().contains("Event") {
                    // 将Event修改为DefaultNone，其他不变
                    let mut new_tk = TokenStream::new();
                    let _ = meta.tokens.clone().into_iter().for_each(|token| {
                        if let TokenTree::Ident(ident) = token {
                            let new_ident = if ident.to_string() == "Event" {
                                Ident::new("DefaultNone", Span::call_site())
                            } else {
                                ident
                            };
                            new_tk.extend(vec![TokenTree::Ident(new_ident)]);
                        } else {
                            new_tk.append(token);
                        }
                    });

                    meta.tokens = new_tk;

                    // 将修改后的Meta赋值回Attribute
                    *attr = Attribute {
                        meta: Meta::List(meta.clone()),
                        ..attr.clone()
                    }
                }
            }
        }
        // 检查是否有Event::None，没有则添加
        if !(new_item
            .variants
            .iter()
            .any(|var| var.ident.to_string().eq("None")))
        {
            new_item.variants.push(parse_quote! { None });
        }

        new_item.to_token_stream()
    }
}
