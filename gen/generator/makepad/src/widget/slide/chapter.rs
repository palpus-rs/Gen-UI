use crate::inherits_widget;
use crate::widget::view::ViewProps;
use crate::{
    props_to_token,
    widget::{DynProps, StaticProps},
    ToToken,
};
use proc_macro2::TokenStream;
use std::fmt::Display;

inherits_widget!(SlideChapterProps, ViewProps);
