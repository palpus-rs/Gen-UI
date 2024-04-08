use gen_parser::{PropsKey, Script, Value};
use proc_macro2::TokenStream;
use syn::Stmt;

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
pub enum ScriptHandles {
    Prop(String, String, PropsKey, String, TokenStream, bool),
    Event(String, String, PropsKey, TokenStream, bool),
    Other(TokenStream),
}

impl ScriptHandles {
    pub fn is_prop_and_get(self) -> (String, String, PropsKey, String, TokenStream, bool) {
        match self {
            ScriptHandles::Prop(tag, id, prop, ident, code, is_root) => {
                (tag, id, prop, ident, code, is_root)
            }
            _ => panic!("only prop can be get"),
        }
    }
    pub fn is_event_and_get(self) -> (String, String, PropsKey, TokenStream, bool) {
        match self {
            ScriptHandles::Event(tag, id, prop, code, is_root) => (tag, id, prop, code, is_root),
            _ => panic!("only event can be get"),
        }
    }
    pub fn is_other_and_get(self) -> TokenStream {
        match self {
            ScriptHandles::Other(tt) => tt,
            _ => panic!("only other can be get"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ScriptHandle {
    props: Vec<ScriptHandles>,
    events: Vec<ScriptHandles>,
    others: Vec<ScriptHandles>,
}

impl ScriptHandle {
    pub fn set_props(&mut self, props: Vec<ScriptHandles>) {
        self.props = props;
    }
    pub fn set_events(&mut self, events: Vec<ScriptHandles>) {
        self.events = events;
    }
    pub fn set_others(&mut self, others: Vec<ScriptHandles>) {
        self.others = others;
    }
    pub fn get_props(&self) -> &Vec<ScriptHandles> {
        &self.props
    }
    pub fn get_events(&self) -> &Vec<ScriptHandles> {
        &self.events
    }
    pub fn get_others(&self) -> &Vec<ScriptHandles> {
        &self.others
    }
    pub fn push_props(&mut self, prop: ScriptHandles) {
        self.props.push(prop);
    }
    pub fn push_events(&mut self, event: ScriptHandles) {
        self.events.push(event);
    }
    pub fn push_others(&mut self, other: ScriptHandles) {
        self.others.push(other);
    }
    pub fn to_token_stream<P, E, O>(
        self,
        mut p: P,
        mut e: E,
        mut o: O,
    ) -> ((TokenStream, TokenStream), TokenStream, TokenStream)
    where
        P: FnMut(Vec<ScriptHandles>) -> (TokenStream, TokenStream),
        E: FnMut(Vec<ScriptHandles>) -> TokenStream,
        O: FnMut(Vec<ScriptHandles>) -> TokenStream,
    {
        let ScriptHandle {
            props,
            events,
            others,
        } = self;
        (p(props), e(events), o(others))
    }
}

#[derive(Debug, Clone)]
pub struct ScriptBuilder {
    pub uses: Option<TokenStream>,
    pub props: Option<TokenStream>,
    pub events: Option<TokenStream>,
    pub lifetimes: Option<Vec<LifeTime>>,
    pub others: Option<ScriptHandle>,
    // Widget标识（是Widget对象的名称）
    pub target: String,
    pub is_component: bool,
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
    pub fn get_others(&self) -> Option<&ScriptHandle> {
        self.others.as_ref()
    }
    pub fn get_others_mut(&mut self) -> Option<&mut ScriptHandle> {
        self.others.as_mut()
    }
    pub fn get_lifetime_mut(&mut self) -> Option<&mut Vec<LifeTime>> {
        self.lifetimes.as_mut()
    }
    pub fn others_to_token_stream<F>(&self, mut f: F) -> TokenStream
    where
        F: FnMut(Option<&ScriptHandle>) -> TokenStream,
    {
        f(self.get_others())
    }
    // pub fn to_token_stream(self, extends: [bool; 5]) -> TokenStream {
    //     let sections = [
    //         self.uses,
    //         self.props,
    //         self.events,
    //         self.lifetimes
    //             .map(|lts| lts.into_iter().map(|lt| lt.to_token_stream()).collect()),
    //         self.others.,
    //     ];

    //     sections
    //         .iter()
    //         .enumerate()
    //         .filter_map(|(index, section)| {
    //             if extends[index] {
    //                 section.as_ref()
    //             } else {
    //                 None
    //             }
    //         })
    //         .fold(TokenStream::new(), |mut acc, ts| {
    //             acc.extend(ts.clone());
    //             acc
    //         })
    // }
}
