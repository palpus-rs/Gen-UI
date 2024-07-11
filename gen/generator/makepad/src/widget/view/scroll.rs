//! - ScrollXView
//! - ScrollYView
//! - ScrollXYView

use proc_macro2::TokenStream;

use std::fmt::Display;

use crate::{
    props_to_token,
    widget::{DynProps, StaticProps},
    ToToken,
};

use super::ViewProps;

#[derive(Debug, Clone, Default)]
pub struct ScrollXViewProps(pub ViewProps);

impl DynProps for ScrollXViewProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        ViewProps::prop_bind(prop, value, is_prop, ident)
    }
}

impl StaticProps for ScrollXViewProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        Self(ViewProps::props(props))
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        self.0.prop(prop_name, value)
    }
}

impl Display for ScrollXViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

props_to_token!(ScrollXViewProps);

#[derive(Debug, Clone, Default)]
pub struct ScrollYViewProps(pub ViewProps);

impl DynProps for ScrollYViewProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        ViewProps::prop_bind(prop, value, is_prop, ident)
    }
}

impl StaticProps for ScrollYViewProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        Self(ViewProps::props(props))
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        self.0.prop(prop_name, value)
    }
}

impl Display for ScrollYViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

props_to_token!(ScrollYViewProps);

#[derive(Debug, Clone, Default)]
pub struct ScrollXYViewProps(pub ViewProps);

impl DynProps for ScrollXYViewProps {
    fn prop_bind(
        prop: &gen_parser::PropsKey,
        value: &gen_parser::Value,
        is_prop: bool,
        ident: &str,
    ) -> proc_macro2::TokenStream {
        ViewProps::prop_bind(prop, value, is_prop, ident)
    }
}

impl StaticProps for ScrollXYViewProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized,
    {
        Self(ViewProps::props(props))
    }

    fn prop(&mut self, prop_name: &str, value: &gen_parser::Value) -> () {
        self.0.prop(prop_name, value)
    }
}

impl Display for ScrollXYViewProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

props_to_token!(ScrollXYViewProps);
