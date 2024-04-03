use gen_converter::{model::Model, strategy::id};
use proc_macro2::TokenStream;

pub mod error;
pub mod gen;
pub mod prop;
pub mod utils;
pub mod widget;

#[derive(Debug)]
pub struct Makepad(pub TokenStream);

impl Makepad {
    pub fn ast(mut model: Model) -> Self {
        // 处理template部分
        let template = model.get_template_mut();

        let _ = id(&mut model, gen::id());
        // 处理script部分

        // 处理style部分
        todo!()
    }
}
