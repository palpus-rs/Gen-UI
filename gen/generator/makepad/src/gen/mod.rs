mod id;
mod class;
mod inherits;
mod style;
mod script;
mod template;

use gen_utils::common::token_stream_to_tree;
pub use id::*;
pub use class::*;
pub use inherits::*;
use proc_macro2::{TokenStream, TokenTree};
pub use style::*;
pub use script::*;
pub use template::*;

pub trait ToToken {
    fn to_token_stream(&self) -> TokenStream;
    fn to_token_trees(&self) -> Vec<TokenTree>{
        token_stream_to_tree(self.to_token_stream())
    }
}