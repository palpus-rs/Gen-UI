use proc_macro2::TokenStream;
use syn::ItemStruct;

use crate::widget::BuiltIn;

pub struct WidgetHandler;

impl WidgetHandler {
    pub fn prop_ptr(prop_ptr: &ItemStruct, inherit: &BuiltIn) -> TokenStream{
        // 将GenUI的结构体转为Makepad的属性结构体
        let tk = inherit.to_token_stream(prop_ptr);


        todo!("{:#?}",tk.to_string())
    }
}