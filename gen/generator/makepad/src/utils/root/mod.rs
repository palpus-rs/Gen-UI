use gen_utils::common::*;
use proc_macro2::TokenTree;

/// generate `impl AppMain for xxx{...}`
/// code: token_group
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

/// generate `impl MatchEvent for xxx{...}`
/// code: token_group
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

/// generate `impl LiveRegister for xxx{ fn live_register(&mut Cx) {...} }`
pub fn impl_live_register(target: TokenTree, code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("impl"),
        token_tree_ident("LiveRegister"),
        token_tree_ident("for"),
        target,
        token_tree_group(vec![
            token_tree_ident("fn"),
            token_tree_ident("live_register"),
            token_tree_group_paren(vec![
                token_tree_punct_alone('&'),
                token_tree_ident("mut"),
                token_tree_ident("Cx"),
            ]),
            token_tree_group(code),
        ]),
    ]
}

/// generate `crate::xxx::live_design(cx)`
pub fn makepad_widgets_register(target: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident("crate"),
        token_tree_punct_joint(':'),
        token_tree_ident(target),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_ident("live_design"),
        token_tree_group_paren(vec![token_tree_ident("cx")]),
    ]
}


pub fn handle_startup(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_startup"),
        token_tree_group_paren(vec![
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

pub fn handle_shutdown(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_shutdown"),
        token_tree_group_paren(vec![
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
        token_tree_group_paren(vec![
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

/// generate `fn handle_event(&mut self, cx: &mut Cx, event: &Event)` in AppMain trait
pub fn handle_event(code: Vec<TokenTree>) -> TokenTree {
    token_tree_group(vec![
        token_tree_ident("fn"),
        token_tree_ident("handle_event"),
        token_tree_group_paren(vec![
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

/// generate `self.match_event(cx, event);`
pub fn self_match_event() -> Vec<TokenTree> {
    vec![
        token_tree_ident("self"),
        token_tree_punct_joint('.'),
        token_tree_ident("match_event"),
        token_tree_group_paren(vec![
            token_tree_ident("cx"),
            token_tree_punct_alone(','),
            token_tree_ident("event"),
        ]),
    ]
}

/// generate `app_main!(XXX);`
pub fn macro_app_main(target: TokenTree) -> Vec<TokenTree> {
    vec![
        token_tree_ident("app_main"),
        token_tree_punct_alone('!'),
        token_tree_group_paren(vec![target]),
        token_tree_punct_alone(';')
    ]
}