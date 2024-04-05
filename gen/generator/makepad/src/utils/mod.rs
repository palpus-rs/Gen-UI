//! this utils module is for general utilities that are used in the generator
//! which is helpful for gen makepad ast

use gen_utils::common::*;
use proc_macro2::TokenTree;

/// generate `use makepad_widgets::*;`
pub fn use_makepad_widget_all() -> Vec<TokenTree> {
    vec![
        token_tree_ident("use"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("*"),
    ]
}

/// generate `live_design!`
pub fn live_design_macro() -> Vec<TokenTree> {
    vec![token_tree_ident("live_design"), token_tree_punct_alone('!')]
}

/// generate `import makepad_widgets::base::*;`
pub fn import_makepad_widgets_base() -> Vec<TokenTree> {
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
pub fn import_makepad_widgets_theme_desktop_dark() -> Vec<TokenTree> {
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

pub fn derive_live_livehook() -> Vec<TokenTree> {
    vec![
        token_tree_ident("Live"),
        token_tree_punct_alone(','),
        token_tree_ident("LiveHook"),
        token_tree_punct_alone(','),
    ]
}

pub fn derive_default_none() -> Vec<TokenTree> {
    vec![token_tree_ident("DefaultNone"), token_tree_punct_alone(',')]
}

pub fn impl_app_main(target: TokenTree, code: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut fn_tt = vec![
        token_tree_ident("impl"),
        token_tree_ident("AppMain"),
        token_tree_ident("for"),
        target,
    ];
    fn_tt.extend(code);
    fn_tt
}

pub fn impl_match_event(target: TokenTree, code: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut fn_tt = vec![
        token_tree_ident("impl"),
        token_tree_ident("MatchEvent"),
        token_tree_ident("for"),
        target,
    ];
    fn_tt.extend(code);
    fn_tt
}

pub fn impl_live_register(target: TokenTree, code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("impl"),
        token_tree_ident("LiveRegister"),
        token_tree_ident("for"),
        target,
        token_tree_ident("fn"),
        token_tree_ident("live_register"),
        token_tree_group(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Cx"),
        ]),
        token_tree_group(code),
    ]
}

pub fn makepad_widgets_register() -> Vec<TokenTree> {
    vec![
        token_tree_ident("crate"),
        token_tree_punct_joint(':'),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("live_design"),
        token_tree_group(vec![token_tree_ident("cx")]),
    ]
}

pub fn handle_startup(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_startup"),
        token_tree_group(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("self"),
            token_tree_punct_alone(','),
            token_tree_ident("cx"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Cx"),
        ]),
        token_tree_group(code),
    ])
}

pub fn handle_actions(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_actions"),
        token_tree_group(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("self"),
            token_tree_punct_alone(','),
            token_tree_ident("cx"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Cx"),
            token_tree_punct_alone(','),
            token_tree_ident("actions"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("Actions"),
        ]),
        token_tree_group(code),
    ])
}

pub fn handle_event(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_event"),
        token_tree_group(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("self"),
            token_tree_punct_alone(','),
            token_tree_ident("cx"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Cx"),
            token_tree_punct_alone(','),
            token_tree_ident("event"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("Event"),
        ]),
        token_tree_group(code),
    ])
}

pub fn self_match_event() -> Vec<TokenTree> {
    vec![
        token_tree_ident("self"),
        token_tree_punct_joint('.'),
        token_tree_ident("match_event"),
        token_tree_group(vec![
            token_tree_ident("cx"),
            token_tree_punct_alone(','),
            token_tree_ident("event"),
        ]),
    ]
}

/// generate makepad dsl
/// return TokenTree::Group
pub fn dsl() -> TokenTree {
    token_tree_group(vec![])
}
