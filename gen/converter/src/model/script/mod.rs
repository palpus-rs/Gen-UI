use std::fmt::Display;

use gen_parser::Script;
use proc_macro2::TokenStream;
use quote::ToTokens;

pub type ConvertScript  = Script;

/// GenUI内置生命周期事件
/// 目前只设置两种事件
#[derive(Debug, Clone)]
pub enum LifeTime{
    StartUp(TokenStream),
    ShutDown(TokenStream),
}

impl LifeTime {
    pub fn to_token_stream(self) -> TokenStream {
        match self {
            LifeTime::StartUp(tt) => tt,
            LifeTime::ShutDown(tt) => tt,
        }
    }
}