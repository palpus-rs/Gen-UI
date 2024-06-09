use std::{hash::Hash, io::Write};

use gen_converter::model::{script::ScriptModel, Model, Source};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse2;

use crate::utils::create_file;

#[derive(Debug, Clone)]
pub struct RsFile {
    pub source: Source,
    pub content: TokenStream,
}

impl PartialEq for RsFile {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl Eq for RsFile {}

impl Hash for RsFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
    }
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
    pub fn content(&self) -> TokenStream {
        let origin_content = self.content.clone();
        if origin_content.is_empty() {
            return origin_content;
        } else {
            // check source name is mod? true => to block and return stmts in block
            if self.source.source_name_lower().eq("mod") {
                let content = parse2::<syn::Block>(origin_content).unwrap();
                content
                    .stmts
                    .into_iter()
                    .fold(TokenStream::new(), |mut acc, item| {
                        acc.extend(item.to_token_stream());
                        acc
                    })
            } else {
                origin_content
            }
        }
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
