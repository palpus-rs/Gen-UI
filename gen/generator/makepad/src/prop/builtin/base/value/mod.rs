mod live;

pub use live::*;
use quote::ToTokens;
use syn::Field;

#[derive(Clone, Debug)]
pub enum MakepadValue {
    Layout,
    Walk,
    Live(LiveValueType),
    Rust,
}

impl From<&Field> for MakepadValue {
    fn from(value: &Field) -> Self {
        let ident = value.ty.to_token_stream().to_string();
        ident.as_str().into()
    }
}

impl From<&str> for MakepadValue {
    fn from(value: &str) -> Self {
        LiveValueType::try_from(value)
            .map(MakepadValue::Live)
            .unwrap_or_else(|_| match value {
                "Layout" => MakepadValue::Layout,
                "Walk" => MakepadValue::Walk,
                _ => MakepadValue::Rust,
            })
    }
}

impl From<&String> for MakepadValue {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}
