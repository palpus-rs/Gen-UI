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

pub fn makepad_widgets_register(target: &str) -> Vec<TokenTree> {
    vec![
        token_tree_ident("crate"),
        token_tree_punct_joint(':'),
        token_tree_ident(target),
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

pub fn macro_app_main(target: TokenTree) -> Vec<TokenTree> {
    vec![
        token_tree_ident("app_main"),
        token_tree_punct_alone('!'),
        token_tree_group_paren(vec![target]),
    ]
}

pub fn apply_over_and_redraw(
    ui: Option<&str>,
    tag: String,
    id: String,
    pv: Vec<TokenTree>,
) -> Vec<TokenTree> {
    let mut f = vec![token_tree_ident("self"), token_tree_punct_joint('.')];
    if ui.is_some() {
        f.push(token_tree_ident(ui.unwrap()));
        f.push(token_tree_punct_alone('.'));
    }

    f.extend(vec![
        token_tree_ident(&tag),
        token_tree_group_paren(vec![
            token_tree_ident("id"),
            token_tree_punct_alone('!'),
            token_tree_group_paren(vec![token_tree_ident(&id)]),
            token_tree_punct_alone('.'),
            token_tree_ident("apply_over_and_redraw"),
            token_tree_group_paren(vec![
                token_tree_ident("cx"),
                token_tree_punct_alone(','),
                token_tree_ident("live"),
                token_tree_punct_alone('!'),
                token_tree_group(pv),
            ]),
        ]),
        token_tree_punct_alone(';'),
    ]);
    f
}

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

/// generate `live!{ //.. }`
pub fn live_macro(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("live"),
        token_tree_punct_alone('!'),
        token_tree_group(code),
    ]
}

/// `#[derive(Debug, Clone, Default)]`
fn derive_macros(marcos: Vec<&str>) -> Vec<TokenTree> {
    // let len  = marcos.len();
    let mut marcos_tks = Vec::new();
    marcos.iter().enumerate().for_each(|(i, v)| {
        marcos_tks.push(token_tree_ident(v));
        if i != marcos.len() {
            marcos_tks.push(token_tree_punct_alone(','))
        }
    });
    vec![
        token_tree_punct_alone('#'),
        token_tree_group_bracket(vec![
            token_tree_ident("derive"),
            token_tree_group_paren(marcos_tks),
        ]),
    ]
}

/// ```
/// #[derive(Debug, Clone, Default)]
/// struct Instance {
///   pub field: Type,
/// }
/// ```
pub fn instance(kvs: Vec<TokenTree>) -> Vec<TokenTree> {
    let mut tk = derive_macros(vec!["Debug", "Clone", "Default"]);
    tk.extend(vec![
        token_tree_ident("struct"),
        token_tree_ident("Instance"),
        token_tree_group(kvs),
    ]);
    tk
}

/// generate `impl xxx{...}`
pub fn impl_target(target: &str, code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("impl"),
        token_tree_ident(target),
        token_tree_group(code),
    ]
}

/// generate `fn new()->Self{}`
pub fn instance_new_fn(code: Vec<TokenTree>) -> Vec<TokenTree> {
    vec![
        token_tree_ident("fn"),
        token_tree_ident("new"),
        token_tree_group_paren(vec![]),
        token_tree_punct_joint('-'),
        token_tree_punct_joint('>'),
        token_tree_ident("Self"),
        token_tree_group(code),
    ]
}

/// generate `...init code...; Self{...}`
pub fn instance_return_self(mut init: Vec<TokenTree>, code: Vec<TokenTree>) -> Vec<TokenTree> {
    init.extend(vec![token_tree_ident("Self"), token_tree_group(code)]);
    init
}

/// generate `pub field: Type,`
pub fn struct_field_type(field: &str, ty: TokenTree) -> Vec<TokenTree> {
    vec![
        token_tree_ident("pub"),
        token_tree_ident(field),
        token_tree_punct_alone(':'),
        ty,
        token_tree_punct_alone(','),
    ]
}

/// generate makepad dsl
/// return TokenTree::Group
pub fn dsl() -> TokenTree {
    token_tree_group(vec![])
}
