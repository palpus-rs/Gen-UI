use crate::model::{Model, TemplateModel};
use gen_utils::error::Errors;
/// 在GenUI中inherits是直接写在`<component>`标签上的
/// 例如：`<component inherits="view">`
/// 而component标签是GenUI中独有的特殊标签，用于定义一个组件
pub fn inherits<F>(model: &mut Model, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&mut TemplateModel) -> (),
{
    if !model.has_template() {
        return Err(Errors::StrategyNoTemplateStyles);
    }
    let template = model.get_template_mut().unwrap();
    f(template);
    Ok(())
}
