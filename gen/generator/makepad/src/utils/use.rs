use gen_utils::common::*;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// generate `use makepad_widgets::*;`
pub fn use_makepad_widget_all() -> Vec<TokenTree> {
    vec![
        token_tree_ident("use"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_punct_alone('*'),
        token_tree_punct_alone(';'),
    ]
}

// /// generate `import makepad_widgets::base::*;`
// pub fn import_makepad_widgets_base() -> TokenStream {
//     quote! {
//         import makepad_widgets::base::*;
//     }
// }

// /// generate `import makepad_widgets::theme_desktop_dark::*;`
// pub fn import_makepad_widgets_theme_desktop_dark() -> TokenStream {
//     quote!{
//         import makepad_widgets::theme_desktop_dark::*;
//     }
// }
