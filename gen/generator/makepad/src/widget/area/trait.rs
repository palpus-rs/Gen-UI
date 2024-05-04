use gen_converter::model::script::PropFn;
use gen_utils::common::token_stream_to_tree;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Pat, Stmt};

use crate::{utils::apply_over_and_redraw, widget::BuiltIn};

pub fn draw_walk(draw_walk: &Option<Vec<PropFn>>) -> TokenStream {
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

    quote! {
        cx.begin_turtle(walk, self.layout);
        #tk
        cx.end_turtle();
        DrawStep::done()
    }
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
