use gen_converter::model::TemplateModel;

/// 对于Makepad来说，同样也存在继承的思想
/// 不过在Makepad中则是在结构体中使用#[deref]来实现
/// ```
/// #[derive(Live, Widget, LiveHook)]
/// pub struct DefineWView {
///    #[deref]
///    pub inherits: View,
/// }
/// ```
/// 所以在这个策略器中只要简单的创建出一个对应的TemplateModel中的prop_ptr即可
/// 这个字段需要使用GenUI的Prop trait来实现
/// 但在这里并不需要实现，而是放在生成代码的地方实现这里即可（即在Makepad的to_string方法中设置）
/// 因为策略器只是对GenUI的AST的Model部分进行补充处理
pub fn inherits() -> impl FnMut(&mut TemplateModel) -> () {
    return |_model| {};
}
