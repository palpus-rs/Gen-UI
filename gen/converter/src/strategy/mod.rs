//！对Model处理的策略器
mod id;
mod class;
mod inherits;
mod style;
mod script;

pub use id::id;
pub use class::class;
pub use inherits::inherits;
pub use style::style;
pub use script::script;