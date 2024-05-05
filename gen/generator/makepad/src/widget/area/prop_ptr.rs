use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use crate::{utils::struct_field, widget::utils::quote_makepad_widget_struct, ToToken};
pub struct AreaPropPtr(pub ItemStruct);

impl From<&ItemStruct> for AreaPropPtr {
    fn from(value: &ItemStruct) -> Self {
        // 将GenUI的结构体转为Makepad的属性结构体
        let mut new_item = quote_makepad_widget_struct(value);
        // 设置#[deref]给当前的属性结构体
        if let Fields::Named(fields) = &mut new_item.fields {
            // add Area
            fields
                .named
                .push(struct_field(vec!["redraw", "rust"], "area", "Area"));
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
