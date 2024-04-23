use std::fmt::Display;

use gen_utils::common::token_tree_ident;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::ToToken;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}
