use proc_macro2::TokenStream;

pub struct Imports(TokenStream);


/// LiveDesign中包含了Makepad DSL的模板部分，这个部分是必须的
/// 它由大量虚拟Widget节点组成
pub struct LiveDesign{
    imports: Imports
}