use std::collections::HashMap;

use gen_parser::{PropsKey, Value};

use crate::{
    error::Errors,
    model::{prop::ConvertStyle, Model, TemplateModel},
};

/// 对TemplateModel(模板部分)的class的处理的策略器
/// - 提供整个TemplateModel的可变引用
/// - 提供整个Model(文件模型)的style(样式)中匹配class的部分
/// 如果模板不存在，则不会执行
pub fn class<F>(model: &mut Model, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&mut TemplateModel, HashMap<PropsKey, Value>) -> (),
{
    if !(model.has_template() && model.has_styles()) {
        return Err(Errors::StrategyNoTemplateStyles);
    }

    let class_value = model
    .get_template()
    .unwrap()
    .get_class()
    .ok_or(Errors::StrategyNoTemplateId)?;

    // class 可能是绑定值，也可能是简单的（unknown）字符串形式
    let class_style = if let Some(class) = class_value.is_unknown_and_get(){
         { model.get_styles().unwrap().get(class) }
        .unwrap_or(&HashMap::new())
        .clone()
    }else{
        // 绑定值则需要在script部分中遍历查找AST节点
        // 但即使是绑定值，也只可能存在最终结果为字符串或字符串数组的情况（String或Vec<String>）
        let script = model.get_script().ok_or(Errors::StrategyNoScript)?;
        
    };
   
    let mut template = model.get_template_mut().unwrap();

    f(&mut template, class_style);
    Ok(())
}
