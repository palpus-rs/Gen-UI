use gen_utils::common::*;
use proc_macro2::TokenTree;

/// generate `fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep`
pub fn draw_walk(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("fn"),
        token_tree_ident("draw_walk"),
        token_tree_group_paren(vec![
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("self"),
            token_tree_punct_alone(','),
            token_tree_ident("cx"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Cx2d"),
            token_tree_punct_alone(','),
            token_tree_ident("scope"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Scope"),
            token_tree_punct_alone(','),
            token_tree_ident("walk"),
            token_tree_punct_alone(':'),
            token_tree_ident("Walk"),
        ]),
        token_tree_punct_joint('-'),
        token_tree_punct_joint('>'),
        token_tree_ident("DrawStep"),
        token_tree_group(code),
    ]
}

/// generate `fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope)`
pub fn handle_event_widget(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
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
            token_tree_punct_alone(','),
            token_tree_ident("scope"),
            token_tree_punct_alone(':'),
            token_tree_punct_alone('&'),
            token_tree_ident("mut"),
            token_tree_ident("Scope"),
        ]),
        token_tree_group(code),
    ]
}