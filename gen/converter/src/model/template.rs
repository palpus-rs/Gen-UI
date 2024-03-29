use std::borrow::Cow;

use super::{ConvertProp,ModelAction};

/// # The Model of template
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateModel<'a>{
    /// id
    special: Option<Cow<'a,str>>,
    /// class
    contexts: Option<Vec<Cow<'a,str>>>,
    /// tag name
    tag_name: Cow<'a,str>,
    /// tag props
    props: Option<Vec<ConvertProp<'a>>>,
    /// tag actions
    actions: Option<Vec<ModelAction>>,
    /// children tag model
    children: Option<Vec<TemplateModel<'a>>>,
    /// is root
    is_root: bool,
    /// inherits
    inherits: Option<Cow<'a,str>>,
}
