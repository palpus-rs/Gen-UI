//! this utils module is for general utilities that are used in the generator
//! which is helpful for gen makepad ast

use proc_macro2::{Group, TokenTree};
use gen_utils::common::*;
use syn::token;

/// generate `use makepad_widgets::*;`
pub fn use_makepad_widget_all()->Vec<TokenTree>{
    vec![
        token_tree_ident("use"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("*"),
    ]
}

/// generate `live_design!`
pub fn live_design_macro()->Vec<TokenTree>{
    vec![
        token_tree_ident("live_design"),
        token_tree_punct_alone('!'),
    ]
}

/// generate `import makepad_widgets::base::*;`
pub fn import_makepad_widgets_base()->Vec<TokenTree>{
    vec![
        token_tree_ident("import"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("base"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_punct_joint('*'),
        token_tree_punct_alone(';'),
    ]
}

/// generate `import makepad_widgets::theme_desktop_dark::*;`
pub fn import_makepad_widgets_theme_desktop_dark()->Vec<TokenTree>{
    vec![
        token_tree_ident("import"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("theme_desktop_dark"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_punct_joint('*'),
        token_tree_punct_alone(';'),
    ]
}


/// generate makepad dsl
/// return TokenTree::Group
pub fn dsl()->TokenTree{
    token_tree_group(vec![

    ])
}