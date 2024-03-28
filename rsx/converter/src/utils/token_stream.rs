use proc_macro2::Span;
use syn::{Ident, PathSegment};

pub fn build_path_segment(ident: &str) -> PathSegment {
    PathSegment {
        ident: Ident::new(ident, Span::call_site()),
        arguments: Default::default(),
    }
}
