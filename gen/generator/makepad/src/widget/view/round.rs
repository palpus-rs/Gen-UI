use proc_macro2::TokenStream;
use super::ViewProps;
use crate::inherits_view;
use crate::{
    props_to_token,
    widget::{DynProps, StaticProps},
    ToToken,
};
use std::fmt::Display;

inherits_view!(RoundedViewProps);

inherits_view!(RoundedShadowViewProps);