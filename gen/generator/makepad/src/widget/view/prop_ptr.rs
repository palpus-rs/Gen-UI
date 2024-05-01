use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemStruct, Meta};

use crate::ToToken;

pub struct ViewPropPtr(pub ItemStruct);

impl From<&ItemStruct> for ViewPropPtr {
    fn from(value: &ItemStruct) -> Self {
        // 将GenUI的结构体转为Makepad的属性结构体
        let ItemStruct {
            mut attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        } = ItemStruct::from(value.clone());
        // [去除GenUI标记的Prop derive宏]-----------------------
        // 修改为#[derive(Live, LiveHook, Widget)]
        let _ = attrs.iter_mut().map(|item| {
            if let Meta::List(meta) = &mut item.meta {
                if meta.tokens.to_string().contains("Prop") && meta.path.is_ident("derive") {
                    // 修改为#[derive(Live, LiveHook, Widget)]
                    meta.tokens = parse_quote! {Live, LiveHook, Widget};
                }
            }
        });

        ViewPropPtr(ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        })
    }
}

impl ToToken for ViewPropPtr {
    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }
}
