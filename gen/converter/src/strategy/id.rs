use std::collections::HashMap;

use gen_parser::{PropsKey, Value};

use crate::{
    error::Errors,
    model::{Model, TemplateModel},
};

/// 对TemplateModel(模板部分)的id的处理的策略器
/// - 提供整个TemplateModel的可变引用
/// - 提供整个Model(文件模型)的style(样式)中匹配id的部分
/// 如果模板不存在，则不会执行
pub fn id<F>(model: &mut Model, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&mut TemplateModel, HashMap<PropsKey, Value>) -> (),
{
    if !(model.has_template() && model.has_styles()) {
        return Err(Errors::StrategyNoTemplateStyles);
    }

    let id = model
        .get_template()
        .unwrap()
        .get_id()
        .ok_or(Errors::StrategyNoTemplateId)?;
    let id_style = { model.get_styles().unwrap().get(id) }
        .unwrap_or(&HashMap::new())
        .clone();
    let mut template = model.get_template_mut().unwrap();

    f(&mut template, id_style);
    Ok(())
}
