//! pub trait LiveHook {
//!     //fn before_live_design(_cx:&mut Cx){}
//!
//!     fn apply_value_unknown(&mut self, cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
//!         if !nodes[index].origin.node_has_prefix() {
//!             cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
//!         }
//!         nodes.skip_node(index)
//!     }
//!
//!     fn apply_value_instance(&mut self, _cx: &mut Cx, _apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
//!         nodes.skip_node(index)
//!     }
//!
//!     fn skip_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode])->Option<usize>{None}
//!     fn before_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]){}
//!     fn after_apply(&mut self, _cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {}
//!     fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
//!         match &apply.from{
//!             ApplyFrom::NewFromDoc{..}=>{self.after_new_from_doc(cx);self.after_apply_from_doc(cx);}
//!             ApplyFrom::UpdateFromDoc{..}=>{self.after_update_from_doc(cx);self.after_apply_from_doc(cx);}
//!             _=>()
//!         }
//!     }
//!     fn after_new_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_update_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_apply_from_doc(&mut self, _cx:&mut Cx){}
//!     fn after_new_before_apply(&mut self, _cx: &mut Cx) {}
//! }

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// LiveHookTrait是一个trait，它包含了LiveHook的所有方法
/// 它被使用在Widget的实现中
#[derive(Debug, Default, Clone)]
pub struct LiveHookTrait{
    pub before_live_design: Option<TokenStream>,
    pub apply_value_unknown: Option<TokenStream>,
    pub apply_value_instance: Option<TokenStream>,
    pub skip_apply: Option<TokenStream>,
    pub before_apply: Option<TokenStream>,
    pub after_apply: Option<TokenStream>,
    pub after_apply_from: Option<TokenStream>,
    pub after_new_from_doc: Option<TokenStream>,
    pub after_update_from_doc: Option<TokenStream>,
    pub after_apply_from_doc: Option<TokenStream>,
    pub after_new_before_apply: Option<TokenStream>,
}

impl  LiveHookTrait{
    pub fn before_live_design(&mut self, tk: TokenStream) -> () {
        self.before_live_design = Some(quote! {
            fn before_live_design(_cx:&mut Cx){
                #tk
            }
        });
    }
    pub fn apply_value_unknown(&mut self, tk: TokenStream) -> () {
        self.apply_value_unknown = Some(quote! {
            fn apply_value_unknown(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        });
    }
    pub fn apply_value_instance(&mut self, tk: TokenStream) -> () {
        self.apply_value_instance = Some(quote! {
            fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
                #tk
            }
        });
    }
    pub fn skip_apply(&mut self, tk: TokenStream) -> () {
        self.skip_apply = Some(quote! {
            fn skip_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> Option<usize> {
                #tk
            }
        });
    }
    pub fn before_apply(&mut self, tk: TokenStream) -> () {
        self.before_apply = Some(quote! {
            fn before_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                #tk
            }
        });
    }
    pub fn after_apply(&mut self, tk: Option<TokenStream>) -> () {
        if let Some(apply) = tk {
            self.after_apply = Some(quote! {
                fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
                    #apply
                }
            });
        }
    }
    pub fn after_apply_from(&mut self, tk: TokenStream) -> () {
        self.after_apply_from = Some(quote! {
            fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
                #tk
            }
        });
    }
    pub fn after_new_from_doc(&mut self, tk: TokenStream) -> () {
        self.after_new_from_doc = Some(quote! {
            fn after_new_from_doc(&mut self, cx: &mut Cx) {
                #tk
            }
        });
    }
    pub fn after_update_from_doc(&mut self, tk: TokenStream) -> () {
        self.after_update_from_doc = Some(quote! {
            fn after_update_from_doc(&mut self, cx: &mut Cx) {
                #tk
            }
        });
    }
    pub fn after_apply_from_doc(&mut self, tk: TokenStream) -> () {
        self.after_apply_from_doc = Some(quote! {
            fn after_apply_from_doc(&mut self, cx: &mut Cx) {
                #tk
            }
        });
    }
    pub fn after_new_before_apply(&mut self, tk: TokenStream) -> () {
        self.after_new_before_apply = Some(quote! {
            fn after_new_before_apply(&mut self, cx: &mut Cx) {
                #tk
            }
        });
    }
    pub fn to_token_stream(&self, target: Ident) -> TokenStream {
        let before_live_design = self.before_live_design.as_ref();
        let apply_value_unknown = self.apply_value_unknown.as_ref();
        let apply_value_instance = self.apply_value_instance.as_ref();
        let skip_apply = self.skip_apply.as_ref();
        let before_apply = self.before_apply.as_ref();
        let after_apply = self.after_apply.as_ref();
        let after_apply_from = self.after_apply_from.as_ref();
        let after_new_from_doc = self.after_new_from_doc.as_ref();
        let after_update_from_doc = self.after_update_from_doc.as_ref();
        let after_apply_from_doc = self.after_apply_from_doc.as_ref();
        let after_new_before_apply = self.after_new_before_apply.as_ref();

        quote! {
            impl LiveHook for #target{
                #before_live_design
                #apply_value_unknown
                #apply_value_instance
                #skip_apply
                #before_apply
                #after_apply
                #after_apply_from
                #after_new_from_doc
                #after_update_from_doc
                #after_apply_from_doc
                #after_new_before_apply
            }
        }
    }
}