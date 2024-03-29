pub mod prop;

use prop::PropStrategy;
/// Strategies for converting .gen code
/// 遍历整个代码，根据不同的部分，产生一个特定的策略器，然后根据策略器进行转换
/// 关注点在于根据KeyWord产生不同的策略器
pub enum Strategies{
    /// Prop Strategy
    Prop(PropStrategy),
    Script,
    Style,
}