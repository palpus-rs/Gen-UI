use std::collections::HashMap;

use gen_parser::{PropsKey, Value};
use gen_traits::event::Event;
/// ## 单条被注册的事件
/// 例如：
/// ```rust
/// #[derive(Debug, Clone, PartialEq, Event)]
/// pub enum Events{
///     #[name = "click"]
///     Clicked(//内部给到外部的参数),
/// }
/// ```
/// name就是click, event就是Events::Clicked
pub struct ModelEvent<T: Event> {
    pub name: String,
    pub event: Option<T>,
}

/// ## 事件回调集合
/// 用于标识外部传入组件的事件的集合
/// 它由以下部分组成
/// - 事件名称
/// - 事件指针（这个指针只是代表这个事件在代码中赋值的变量名，例如let `btn_click` = || {}， btn_click就是这个指针）
/// - 事件
pub type Callbacks = HashMap<PropsKey, Value>;

/// 默认的组件的事件，即没有任何事件，不提供任何回调
#[derive(gen_macros::Event, Debug, Clone, Default)]
pub enum NoEvent {
    #[default]
    None,
}
