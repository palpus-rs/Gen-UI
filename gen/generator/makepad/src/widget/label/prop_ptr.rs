use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Fields, ItemStruct};

use crate::{utils::struct_field, widget::utils::quote_makepad_widget_struct, ToToken};

pub struct LabelPropPtr(pub ItemStruct);

impl From<&ItemStruct> for LabelPropPtr {
    fn from(value: &ItemStruct) -> Self {
        // 将GenUI的结构体转为Makepad的属性结构体
        let mut new_item = quote_makepad_widget_struct(value);
        // 设置#[deref]给当前的属性结构体
        if let Fields::Named(fields) = &mut new_item.fields {
            // add view
            fields
                .named
                .push(struct_field(vec!["deref"], "label", "Label"));
        }
        LabelPropPtr(new_item)
    }
}

impl ToToken for LabelPropPtr {
    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }
}
