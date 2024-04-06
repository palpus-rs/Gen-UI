//！对Model处理的策略器
mod class;
mod id;
mod inherits;
mod script;
mod style;

pub use class::class;
pub use id::id;
pub use inherits::inherits;
pub use script::{scirpt_builder, script};
pub use style::style;
