use gen_parser::{PropsKey, Value};
use syn::Stmt;

/// 组件属性绑定
#[derive(Debug, Clone)]
pub struct PropFn{
    /// 组件名
    pub widget: String,
    /// 组件id
    pub id: String,
    /// 组件属性
    pub key: PropsKey,
    /// 绑定的属性值（它会索引到script中设置的变量或方法）
    /// 例如：`<div :text="div_text" @click="on_click"></div>`
    /// 这里的`div_text`和`on_click`就是绑定的属性值
    /// 但也可能出现`<div :text="props.div_text" @click="on_click"></div>`,这属于从上层传入的属性
    pub ident: Value,
    /// 绑定的属性值对应的代码
    pub code: Stmt,
    /// 是否是由上一层传入的属性
    pub is_prop: bool
}
