use gen_utils::common::*;
use proc_macro2::TokenTree;

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

/// generate `import makepad_widgets::base::*;`
pub fn import_makepad_widgets_base() -> Vec<TokenTree> {
    vec![
        token_tree_ident("import"),
        token_tree_ident("makepad_widgets"),
        token_tree_punct_joint(':'),
        token_tree_punct_alone(':'),
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
        token_tree_punct_alone(':'),
        token_tree_ident("theme_desktop_dark"),
        token_tree_punct_joint(':'),
        token_tree_punct_joint(':'),
        token_tree_punct_joint('*'),
        token_tree_punct_alone(';'),
    ]
}
