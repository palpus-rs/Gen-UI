use std::{fs::File, io::Write};

use gen::{sc_builder_to_token_stream, template};
use gen_converter::{
    model::Model,
    strategy::{class, id, inherits, script, style},
};
use gen_utils::common::token_stream_to_tree;
use proc_macro2::{TokenStream, TokenTree};

pub mod error;
pub mod gen;
pub mod prop;
pub mod utils;
pub mod widget;
pub mod instance;

pub trait ToToken {
    fn to_token_stream(&self) -> TokenStream;
    fn to_token_trees(&self) -> Vec<TokenTree>{
        token_stream_to_tree(self.to_token_stream())
    }
}

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
       
        ast_tt.extend(template(model.get_special(),model.get_template()));

        // [处理并生成script部分]------------------------------------------------------------------

        if let Ok(sc) = script(
            model,
            gen::r#use(),
            gen::prop(),
            gen::event(),
            gen::lifetime(),
            gen::other(),
        ) {
            let _ = ast_tt.extend(sc_builder_to_token_stream(sc));
        }
        
        Makepad(ast_tt)
        
    }

    pub fn to_token_stream(self) -> TokenStream {
        self.0
    }   
}