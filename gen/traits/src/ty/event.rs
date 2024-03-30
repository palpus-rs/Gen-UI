use crate::event::Event;

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
pub struct EventItem<T: Event> {
    name: String,
    event: T,
}


impl<T: Event> EventItem<T> {
    pub fn new(name: &str, event: T)->Self{
        EventItem{
            name: name.to_string(),
            event,
        }
    }
}