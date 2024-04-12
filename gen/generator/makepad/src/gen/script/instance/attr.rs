use std::default;

use gen_utils::common::trees_to_token_stream;

use crate::{gen::ToToken, utils::derive_marco_wrap};

#[derive(Debug, Clone, PartialEq,Default)]
pub enum Attr {
    /// `#[live]`
    Live,
    /// `#[rust]`
    #[default]
    Rust,
}

impl ToToken for Attr {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        trees_to_token_stream(derive_marco_wrap(match self {
            Attr::Live => "live",
            Attr::Rust => "rust",
        }))
    }
}

#[cfg(test)]
mod attr_test{
    use crate::gen::ToToken;
    use super::Attr;

    #[test]
    fn tk(){
        let attr1 = Attr::Live;
        let attr2 = Attr::Rust;
        assert_eq!("#[live]".to_string(), attr1.to_token_stream().to_string());
        assert_eq!("#[rust]".to_string(), attr2.to_token_stream().to_string());
    }
}