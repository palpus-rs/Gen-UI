use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Attribute, Fields, ItemStruct, Meta};

use crate::{prop::builtin::MakepadValue, utils::struct_field, ToToken};
pub struct AreaPropPtr(pub ItemStruct);

impl From<&ItemStruct> for AreaPropPtr {
    fn from(value: &ItemStruct) -> Self {
        // 将GenUI的结构体转为Makepad的属性结构体
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
        // 设置#[deref]给当前的属性结构体
        if let Fields::Named(fields) = &mut new_item.fields {
            // add Area
            fields
                .named
                .push(struct_field(vec!["deref", "rust"], "area", "Area"));
            // add Layout
            fields
                .named
                .push(struct_field(vec!["layout"], "layout", "Layout"));
            // add Walk
            fields
                .named
                .push(struct_field(vec!["walk"], "walk", "Walk"));
        }

        AreaPropPtr(new_item)
    }
}

impl ToToken for AreaPropPtr {
    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }
}

#[cfg(test)]
mod test_area_prop_ptr {
    use crate::ToToken;

    #[test]
    fn ptr() {
        let item = quote::quote! {
            #[derive(Prop)]
            pub struct AreaPropPtr{
                pub a: u32,
                pub b: String,
            }
        };
        let item = syn::parse2(item).unwrap();
        let ptr = crate::widget::area::prop_ptr::AreaPropPtr::from(&item);
        let token = ptr.to_token_stream();
        let res = "# [derive (Live , LiveHook , Widget)] pub struct AreaPropPtr { # [live] pub a : u32 , # [live] pub b : String , # [deref] # [rust] pub area : Area , # [layout] pub layout : Layout , # [walk] pub walk : Walk }";
        assert_eq!(token.to_string().as_str(), res);
    }
}
