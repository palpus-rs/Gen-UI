/// `(tag_name, id, (action_name, action_var_name))`
pub type BindAction = (String, String, (String, String));

#[derive(Debug, Clone, PartialEq)]
pub struct ModelAction{}