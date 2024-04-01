use std::fmt::Debug;

/// 用于标注属性的特性
pub trait Prop:  Send + Sync + Debug {
    fn clone_box(&self) -> Box<dyn Prop>;
}

impl Clone for Box<dyn Prop> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}