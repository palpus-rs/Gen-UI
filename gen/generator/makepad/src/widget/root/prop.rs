use std::fmt::Display;

use proc_macro2::TokenStream;

use crate::{
    props_to_token,
    widget::{DynProps, StaticProps},
    ToToken,
};

#[derive(Debug, Clone, Default)]
pub struct RootProps;

impl DynProps for RootProps {
    fn prop_bind(
        _prop: &gen_parser::PropsKey,
        _value: &gen_parser::Value,
        _is_prop: bool,
        _ident: &str,
    ) -> proc_macro2::TokenStream {
        TokenStream::new()
    }
}

impl StaticProps for RootProps {
    fn props(_props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        RootProps
    }

    fn prop(&mut self, _prop_name: &str, _value: &gen_parser::Value) -> () {
        ()
    }
}

impl Display for RootProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
props_to_token!(RootProps);
