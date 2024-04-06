use gen_parser::Script;
use proc_macro2::TokenStream;

pub type ConvertScript = Script;

/// GenUI内置生命周期事件
/// 目前只设置两种事件
#[derive(Debug, Clone)]
pub enum LifeTime {
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

#[derive(Debug, Clone)]
pub struct ScriptBuilder {
    pub uses: Option<TokenStream>,
    pub props: Option<TokenStream>,
    pub events: Option<TokenStream>,
    pub lifetimes: Option<Vec<LifeTime>>,
    pub others: Option<TokenStream>,
    // Widget标识（是Widget对象的名称）
    pub target: String,
}

impl ScriptBuilder {
    // pub fn handle<F>(&mut self, f: F) -> ()
    // where F: Fn(&mut ScriptBuilder)->() {
    //     f(self);
    // }
    pub fn has_lifetime(&self) -> bool {
        self.lifetimes.is_some()
    }
    pub fn has_others(&self) -> bool {
        self.others.is_some()
    }
    pub fn get_others(&self) -> Option<&TokenStream> {
        self.others.as_ref()
    }
    pub fn get_others_mut(&mut self)-> Option<&mut TokenStream>{
        self.others.as_mut()
    }
    pub fn get_lifetime_mut(&mut self)-> Option<&mut Vec<LifeTime>>{
        self.lifetimes.as_mut()
    }
    pub fn to_token_stream(self, extends: [bool; 5]) -> TokenStream {
        let sections = [
            self.uses,
            self.props,
            self.events,
            self.lifetimes
                .map(|lts| lts.into_iter().map(|lt| lt.to_token_stream()).collect()),
            self.others,
        ];

        sections
            .iter()
            .enumerate()
            .filter_map(|(index, section)| {
                if extends[index] {
                    section.as_ref()
                } else {
                    None
                }
            })
            .fold(TokenStream::new(), |mut acc, ts| {
                acc.extend(ts.clone());
                acc
            })
    }
}
