//! this utils module is for general utilities that are used in the generator
//! which is helpful for gen makepad ast
mod common;
mod component;
mod macros;
mod root;
mod r#use;

pub use common::*;
pub use component::*;
pub use macros::*;
pub use r#use::*;
pub use root::*;

use gen_utils::common::*;
use proc_macro2::TokenTree;

/// generate makepad dsl
/// return TokenTree::Group
pub fn dsl() -> TokenTree {
    token_tree_group(vec![])
}
