use gen_converter::{model::Model, strategy::{class, id, inherits, style}};
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
        // [这一部分是为了对Model进一步进行处理]-----------------------------------------------------
        // 处理template部分
        let _ = id(&mut model, gen::id());
        let _ = class(&mut model, gen::class());
        let _ = inherits(&mut model, gen::inherits());
        // 处理script部分

        // 处理style部分
        let  _ = style(&mut model, gen::style());
        // [完成处理后这个model就是最终的Model，下面就可以开始生成Makepad AST]-----------------------------------------------------
        todo!("{:#?}", model.script)
    }

    pub fn to_token_stream(&self) -> TokenStream{

        todo!()
    }
}

