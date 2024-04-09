use crate::{
    error::Errors,
    model::{prop::ConvertStyle, Model},
};

pub fn style<F>(model: &mut Model, mut f: F) -> Result<(), Errors>
where
    F: FnMut(&mut ConvertStyle) -> (),
{
    if !model.has_template() {
        return Err(Errors::StrategyNoTemplateStyles);
    }
    match model.get_styles_mut() {
        Some(style) => {
            f(style);
            Ok(())
        }
        None => Err(Errors::StrategyNoStyle),
    }
}
