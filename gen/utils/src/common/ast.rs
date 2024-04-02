use proc_macro2::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};
use syn::Ident;

/// create a new ident
pub fn ident(input: &str) -> Ident {
    Ident::new(input, Span::call_site())
}

/// create a new punct(use between the stmt)
pub fn punct_joint(input: char) -> Punct {
    Punct::new(input, Spacing::Joint)
}

/// create a new punct(use end of the stmt)
pub fn punct_alone(input: char) -> Punct {
    Punct::new(input, Spacing::Alone)
}

pub fn group(tree: Vec<TokenTree>) -> Group {
    let stream = TokenStream::from_iter(tree.into_iter());
    Group::new(Delimiter::Brace, stream)
}

/// create a new token tree ident
/// debug result:
/// ```
/// Ident {
/// sym: use,
/// },
/// ```
pub fn token_tree_ident(input: &str) -> TokenTree {
    TokenTree::Ident(ident(input))
}

pub fn token_tree_punct_joint(input: char) -> TokenTree {
    TokenTree::Punct(punct_joint(input))
}

pub fn token_tree_punct_alone(input: char) -> TokenTree {
    TokenTree::Punct(punct_alone(input))
}

pub fn token_tree_group(tree: Vec<TokenTree>) -> TokenTree {
    TokenTree::Group(group(tree))
}
