use proc_macro2::TokenStream;

pub mod widget;
pub mod prop;
pub mod gen;
pub mod utils;
pub mod error;

pub struct Makepad(pub TokenStream);