use proc_macro2::TokenTree;

use crate::{
    error::Errors,
    model::{Model, TemplateModel},
};


/// 在GenUI中Rust脚本是直接写在`<script>`标签里的
/// 例如：`<script>println!("Hello, World!");</script>`
/// 其实最主要的目的在于为TemplateModel的prop_ptr和event_ptr赋值
/// 这个策略器中包含有多个闭包
/// - use 策略器： 处理use语句，获取所有use语句并返回TokenTree
/// - prop 策略器：获取带有`#[derive(Prop)]`的结构体，并返回TokenTree
/// - event 策略器：获取带有`#[derive(Event)]`的枚举，并返回TokenTree
pub fn script<F>(model: &mut Model, mut use_f: F, mut prop_f: F, mut event_f: F) -> Result<(), Errors>
where
    F: FnMut(&mut TemplateModel) -> TokenTree,
{
    if !model.has_script(){
        return Err(Errors::StrategyNoScript);
    }
    let script = model.get_script().unwrap();

    Ok(())
}