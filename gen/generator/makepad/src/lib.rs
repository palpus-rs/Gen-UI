use std::{fs::File, io::Write};

use gen_converter::{
    model::Model,
    strategy::{class, id, inherits, script, style},
};
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
        let mut ast_tt = TokenStream::new();
        // [这一部分是为了对Model进一步进行处理]-----------------------------------------------------
        // 处理template部分
        let _ = id(&mut model, gen::id());
        let _ = class(&mut model, gen::class());
        let _ = inherits(&mut model, gen::inherits());
        // 处理style部分
        let _ = style(&mut model, gen::style());

        // [完成处理后这个model就是最终的Model，下面就可以开始生成Makepad AST]-----------------------------------------------------
        // 处理script部分
        match script(
            model,
            gen::r#use(),
            gen::prop(),
            gen::event(),
            gen::lifetime(),
            gen::other(),
        ) {
            Ok(tt) => ast_tt.extend(tt),
            Err(_) => (),
        }
        let res = ast_tt.to_string();
        let mut f = File::create("E:/Rust/try/makepad/Gen-UI/gen/tests/release/hello.rs").unwrap();
        let _ = f.write(res.as_bytes());
        todo!("{:#?}", &res);
        // todo!("{:#?}", model.script.unwrap())
    }

    pub fn to_token_stream(&self) -> TokenStream {
        todo!()
    }
}
