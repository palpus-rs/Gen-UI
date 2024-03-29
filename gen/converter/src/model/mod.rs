pub mod action;
pub mod prop;
mod template;

use std::borrow::Cow;

use action::ModelAction;
use prop::ConvertProp;
pub use template::TemplateModel;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model<'a> {
    /// file name of the model also the model struct name
    special: Cow<'a, str>,
    // single model
    template: Option<TemplateModel<'a>>,
    script: Option<ConvertScript>,
    style: Option<ConvertProp<'a>>,
    widget_ref: Option<Cow<'a, str>>,
    props: Option<Vec<ConvertProp>>,
    actions: Option<Vec<BindAction>>,
}