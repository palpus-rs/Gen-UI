use std::num::ParseFloatError;

use gen_utils::common::token_stream_to_tree;

use proc_macro2::TokenTree;
use quote::quote;

use crate::prop::ALIGN;

use super::normal_prop;

pub fn align(value: &str) -> (String, Vec<TokenTree>) {
    todo!()
    // fn handle(value: &str) -> Align {
    //     match value
    //         .split(' ')
    //         .map(|x| x.parse::<f64>())
    //         .collect::<Result<Vec<f64>, ParseFloatError>>()
    //     {
    //         Ok(aligns) => match aligns.len() {
    //             1 => Align {
    //                 x: aligns[0],
    //                 y: aligns[0],
    //             },
    //             2 => Align {
    //                 x: aligns[0],
    //                 y: aligns[1],
    //             },
    //             _ => panic!("{} cannot be converted to Makepad::Align!", value),
    //         },
    //         Err(_) => panic!("{} cannot be converted to Makepad::Align!", value),
    //     }
    // }
    // let align_display = format!("{:?}", handle(value));
    // (
    //     ALIGN.to_string(),
    //     token_stream_to_tree(quote! {#align_display}),
    // )
}
