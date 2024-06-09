use std::collections::HashMap;

use gen_parser::{PropsKey, Value};

use crate::{
    error::Errors,
    model::{Model, TemplateModel},
};

/// 对TemplateModel(模板部分)的class的处理的策略器
/// 目前只处理class为非绑定值的情况
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
    let class_style = if let Some(class) = class_value.is_unknown_and_get() {
        { model.get_styles().unwrap().get(class) }
            .unwrap_or(&HashMap::new())
            .clone()
    } else {
        // 绑定值则需要在script部分中遍历查找AST节点
        // 但即使是绑定值，也只可能存在最终结果为字符串或字符串数组的情况（String或Vec<String>）
        // 但绑定值无法直接在编译阶段获取必须是在运行时获取
        // 虽然直接在GenUI的脚本部分中获取绑定值几乎不可能，但却可以通过GenUI的标识符把其他部分剔除
        // let _script = model.get_script().ok_or(Errors::StrategyNoScript)?;
        todo!("class 策略绑定部分暂不处理，需等到确定所有GenUI标识符后再处理")
    };

    let mut template = model.get_template_mut().unwrap();

    f(&mut template, class_style);
    Ok(())
}
