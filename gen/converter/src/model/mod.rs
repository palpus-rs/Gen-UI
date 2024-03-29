pub mod action;
pub mod prop;
mod template;
mod script;

use std::borrow::Cow;

use action::ModelAction;
use prop::ConvertProp;
pub use template::TemplateModel;

use self::{action::Action, prop::Props, script::ConvertScript};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model<'a> {
    /// file name of the model also the model struct name
    special: Cow<'a, str>,
    // single model
    template: Option<TemplateModel<'a>>,
    script: Option<ConvertScript>,
    style: Option<ConvertProp<'a>>,
    widget_ref: Option<Cow<'a, str>>,
    props: Props<'a>,
    actions: Option<Vec<Action>>,
}