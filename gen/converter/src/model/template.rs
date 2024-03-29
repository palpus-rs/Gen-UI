use std::borrow::Cow;

use gen_parser::ASTNodes;

use crate::keyword::KeyWords;

use super::{prop::Props, ConvertProp, ModelAction};

/// # The Model of template
#[derive(Debug, Clone, PartialEq,Default)]
pub struct TemplateModel{
    /// id
    special: Option<String>,
    /// class
    contexts: Option<Vec<String>>,
    /// tag name
    tag_name: String,
    /// tag props
    props: Option<Vec<ConvertProp>>,
    /// tag actions
    actions: Option<Vec<ModelAction>>,
    /// children tag model
    children: Option<Vec<TemplateModel>>,
    /// is root
    is_root: bool,
    /// inherits
    inherits: Option<String>,
}


pub fn handle_template(template: &ASTNodes)->(TemplateModel, Props){
  if let ASTNodes::Tag(t) = template{
     let mut model = TemplateModel::default();

    
    if t.has_props(){
        for (k,v) in t.get_props().unwrap() {
            // check keyword
            KeyWords::try_from(k.name()).and_then(|prop_name|{
                prop_name.value_prop(v, &mut model)
            }).unwrap_or_else(|_|{
                model.props = Some(vec![v.get_value().to_string()]);
            });
        }
    }

  }
}

/// acturally if the handle_tag() function can run
/// it must have ConvertScript
fn handle_tag(
    t: &Tag,
    styles: Option<&ConvertStyle>,
    is_ref: bool,
) -> (MakepadModel, Vec<BindProp>, Vec<BindAction>) {
    // 1. uppercase the first title case of the tag
    // if can not upper - panic!
    let tag_name = snake_to_camel(t.get_name());
    // 2. add `<` `>` surround the tag
    // 3. add `{` `}` after the tag
    let mut tag_model = MakepadModel::new(&tag_name, is_ref);
    let mut binds = Vec::new();
    let mut actions: Vec<BindAction> = Vec::new();
    // check props
    if t.has_props() {
        let mut has_bind = false;
        let mut has_action = false;
        for prop in t.get_props().unwrap() {
            match PropRole::try_from((tag_name.as_str(), prop)) {
                Ok(p) => {
                    // dbg!(&p);
                    match p {
                        PropRole::Normal(_, _) => tag_model.push_prop(p),
                        PropRole::Bind(k, v) => {
                            has_bind = true;
                            binds.push((tag_name.clone(), String::new(), (k, v)));
                        }
                        PropRole::Function(k, v) => {
                            has_action = true;
                            actions.push((
                                tag_name.clone(),
                                String::new(),
                                (k, v.get_fn_key().to_string()),
                            ));
                            // tag_model.push_action(p)
                        }
                        PropRole::Context(c) => {
                            c.into_iter().for_each(|x| tag_model.push_context(x));
                        }
                        PropRole::Special(s) => tag_model.set_special(s),
                        PropRole::Component(c) => tag_model.set_inherit(Some(c)),
                    }
                }
                Err(e) => panic!("{}", e.to_string()),
            };
        }

        // add special for all binds
        if has_bind {
            match tag_model.get_special() {
                Some(special) => {
                    let _ = binds
                        .iter_mut()
                        .for_each(|bind| bind.1 = special.to_string());
                }
                None => {
                    if !tag_model.is_component() {
                        dbg!(&tag_model);
                        panic!(
                            "the widget(expcet component) which has binds need to add special id"
                        );
                    }
                }
            }
        }
        if has_action {
            match tag_model.get_special() {
                Some(special) => {
                    let _ = actions
                        .iter_mut()
                        .for_each(|action| action.1 = special.to_string());
                }
                None => panic!("the widget which has actions need to add special id"),
            }
        }
    }

    // have styles
    // true: do not need to associate with styles
    // false: need if style exists
    if styles.is_some() {
        let styles = styles.unwrap();
        // when special and context means link , need to patch
        if let Some(links) = tag_model.get_links() {
            for link in links {
                if let Some(sheets) = styles.get(&Cow::Borrowed(link.as_str())) {
                    let _ = sheets.iter().try_for_each(|kv| {
                        PropRole::try_from((&tag_name, kv)).map(|item| tag_model.push_prop(item))
                    });
                }
            }
        }
    }

    // children
    if t.has_children() {
        for child_node in t.get_children().unwrap() {
            match child_node {
                ASTNodes::Tag(child) => {
                    let (child_model, child_binds, child_actions) =
                        handle_tag(*&child, styles, false);
                    tag_model.push_child(child_model);
                    binds.extend(child_binds);
                    actions.extend(child_actions);
                }
                ASTNodes::Comment(_) => (),
                ASTNodes::Style(_) => panic!("{}", "cannot write styles in template node"),
            }
        }
    }

    (tag_model, binds, actions)
}
