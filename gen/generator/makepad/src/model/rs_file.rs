use std::io::Write;

use gen_converter::model::{script::ScriptModel, Model, Source};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::utils::create_file;

#[derive(Debug, Clone)]
pub struct RsFile {
    pub source: Source,
    pub content: TokenStream,
}

impl RsFile {
    pub fn new(source: Source, content: TokenStream) -> Self {
        RsFile { source, content }
    }
    pub fn new_empty(source: Source) -> Self {
        RsFile {
            source,
            content: TokenStream::new(),
        }
    }
    pub fn compile(&self) -> () {
        let mut file = create_file(self.source.compiled_file.as_path());
        file.write_all(self.content.to_string().as_bytes()).unwrap();
    }
}

impl From<Model> for RsFile {
    fn from(value: Model) -> Self {
        if let ScriptModel::Rs(rs) = value.script.as_ref().unwrap() {
            RsFile::new(value.special, rs.to_token_stream())
        } else {
            panic!("Model to RsFile error")
        }
    }
}
