use crate::{widget::StaticProps, ToToken};

#[derive(Debug, Clone, Default)]
pub struct ButtonProps{
    
}

impl StaticProps for ButtonProps {
    fn props(props: &std::collections::HashMap<gen_parser::PropsKey, gen_parser::Value>) -> Self
    where
        Self: Sized {
        todo!()
    }

    fn prop(&mut self, prop_name: &str, value: gen_parser::Value) -> () {
        todo!()
    }
}

impl ToToken for ButtonProps {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        todo!()
    }
}