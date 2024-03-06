mod style;
mod prop;
mod widget;
pub mod value;
pub mod model;


pub use widget::*;
pub use prop::*;
pub use style::*;

use std::fmt::{format, Result};

use parser::{PropsKey, Style, Tag, Value};

use crate::{
    context::{LEFT_ANGLE_BRACKET, RIGHT_ANGLE_BRACKET},
    error::Errors,
    traits::Visitor,
    utils::alphabetic::{surround, uppercase_title},
};

use self::model::{models_to_string, MakepadModel};

pub struct MakepadConverter;

impl Visitor for MakepadConverter {
    fn convert(
        ast: &parser::ParseResult,
        source_name: &str,
    ) -> Result<String, crate::error::Errors> {
        let t_wrap = ast.template();
        let sc_wrap = ast.script();
        let s_wrap = ast.style();
        match t_wrap {
            Some(templates) => {
                let mut ref_tag = true;
                let t_res = templates
                    .into_iter()
                    .map(|t| {
                        let handled_template = Self::convert_template(t,ref_tag);
                        ref_tag = false;
                        return handled_template;
                    })
                    .collect::<Result<Vec<MakepadModel>, Errors>>()?;
                let t_res = models_to_string(t_res);
                let f=  format!("live_design!{{\nimport makepad_widgets::base::*;\nimport makepad_widgets::theme_desktop_dark::*; App = {{ {} }}{{ \n{}\n }} }}",source_name, t_res);
                Ok(f)
            }
            None => Ok(String::new()),
        }
    }

    fn convert_template(t: &parser::ASTNodes,is_ref:bool) -> Result<MakepadModel, crate::error::Errors> {
        match t {
            parser::ASTNodes::Tag(t) => {
                let handled_tag = handle_tag(t,is_ref)?;
                Ok(handled_tag)
            }
            parser::ASTNodes::Comment(c) => todo!(),
            parser::ASTNodes::Style(s) => todo!(),
        }
    }

    fn convert_script(&self, sc: parser::Script) {
        todo!()
    }

    fn convert_style(&self, s: &parser::ASTNodes)-> Result<PropRole,Errors> {
        match s {
            parser::ASTNodes::Style(s) => handle,
            _ => Err(Errors::UnAcceptConvertRange)
        }
    }
}

fn handle_style(s:&Box<Style>)->Result<PropRole,Errors>{
    let style_name = s.get_name();
    match s.get_type() {
        parser::StyleType::Class => style_class(),
        parser::StyleType::Id => todo!(),
        parser::StyleType::Pseudo => todo!(),
    }
}


fn handle_tag(t: &Box<Tag>,is_ref:bool) -> Result<MakepadModel, crate::error::Errors> {
    // 1. uppercase the first title case of the tag
    // 2. add `<` `>` surround the tag
    // 3. add `{` `}` after the tag
    let tag_name = uppercase_title(t.get_name())?;
    let mut tag_model = MakepadModel::new(&tag_name,is_ref);
    // check props
    if t.has_props() {

        for prop in t.get_props().unwrap(){
            match prop_match(&tag_name, prop){
                Ok(p) => {
                    if p.is_special(){
                        tag_model.set_special(p.to_special());
                    }else{
                        tag_model.push_prop(p);
                    }
                },
                Err(e) => return Err(e),
            };
        }
    }
    Ok(tag_model)
    // if  t.has_children(){

    // }
    // Ok(format!("{}{{}}",surround(tag_name, LEFT_ANGLE_BRACKET, RIGHT_ANGLE_BRACKET)))
}



/// Match properties based on the existing components in the current makepad widgets
fn prop_match(tag: &str, prop: (&PropsKey, &Value)) -> Result<PropRole,Errors> {
    match tag {
        "Window" => window(prop.0,prop.1),
        "Button" => button(prop.0,prop.1),
        _ => Err(Errors::UnMatchedWidget),
    }
    
}

#[cfg(test)]
mod test_makepad {
    use parser::{ParseCore, ParseResult, ParseTarget};

    use crate::traits::Visitor;

    use super::MakepadConverter;

    #[test]
    fn convert_t() {
        // example for: window single button
        let input = r#"
        <template>
            <window id="ui">
                <button id="my_button" text="Hello, World" @clicked="btn_click"></button>
            </window>
        </template>

        <script>
        let mut btn_click = || {
            println!("CLICKED!");
        }
        </script>
        "#;
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let _ = MakepadConverter::convert(&ast, "App");
    }
}
