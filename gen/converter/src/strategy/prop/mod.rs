mod normal;
mod bind;
mod function;

pub use normal::NormalProp;
pub use bind::BindProp;
pub use function::FunctionProp;

pub enum PropStrategy{
    Normal(NormalProp),
    Bind(BindProp),
    Function(FunctionProp),
}