#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ConvertScript {
    Rust(String),
    /// need to join('\n')
    RS(Vec<ScriptNode>),
}
