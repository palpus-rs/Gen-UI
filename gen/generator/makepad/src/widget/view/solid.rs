use super::ViewProps;
use proc_macro2::TokenStream;
use crate::inherits_view;
use crate::{
    props_to_token,
    widget::{DynProps, StaticProps},
    ToToken,
};
use std::fmt::Display;

inherits_view!(SolidViewProps);
