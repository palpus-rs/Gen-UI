use crate::{error::Errors, model::{prop::ConvertStyle, Model}};

pub fn style<F>(model: &mut Model, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&mut ConvertStyle) -> (),
{
    if !model.has_template() {
        return Err(Errors::StrategyNoTemplateStyles);
    }
    let style = model.get_styles_mut().unwrap();

    f(style);
    Ok(())
}