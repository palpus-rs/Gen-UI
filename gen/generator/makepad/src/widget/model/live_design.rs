use proc_macro2::TokenStream;

use super::widget::Widget;

/// LiveDesign中包含了Makepad DSL的模板部分，这个部分是必须的
/// 它由大量虚拟Widget节点组成
#[derive(Debug,Default,Clone)]
pub struct LiveDesign{
    imports: Vec<String>,
    tree: Widget
}