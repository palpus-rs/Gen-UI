use std::fmt::Debug;

/// 这个Event trait仅作为一个事件标识存在
/// 主要目的在于快速让GenUI定位到事件
/// 所以这个trait不需要任何方法
pub trait Event: Debug + Send + Sync {
    fn clone_box(&self) -> Box<dyn Event>;
}

impl Clone for Box<dyn Event> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
