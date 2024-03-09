pub mod constants;
pub mod model;
mod prop;
pub mod result;
mod style;
pub mod value;
mod widget;

pub use prop::*;
use quote::quote;
pub use style::*;
use syn::{parse_quote, Local, Stmt};
pub use widget::*;

use std::{borrow::Cow, collections::HashMap, fmt::Display};

use parser::{PropsKey, Style, Value};

use crate::{error::Errors, traits::Visitor, utils::alphabetic::uppercase_title};

use self::model::MakepadModel;

type ConvertStyle<'a> = HashMap<Cow<'a, str>, Cow<'a, HashMap<PropsKey, Value>>>;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ConvertScript {
    Rust(String),
    /// need to join('\n')
    MakepadRS(Vec<Stmt>),
}

impl Display for ConvertScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertScript::Rust(rs) => f.write_str(rs),
            ConvertScript::MakepadRS(stmts) => {
                let block = stmts
                    .into_iter()
                    .map(|stmt| quote! { #stmt }.to_string())
                    .collect::<String>();
                f.write_fmt(format_args!("{}", block))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MakepadConverter<'a> {
    root: Cow<'a, str>,
    template: Option<Vec<MakepadModel>>,
    script: Option<ConvertScript>,
    style: Option<ConvertStyle<'a>>,
}

#[allow(dead_code)]
impl<'a> MakepadConverter<'a> {
    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }
    pub fn has_script(&self) -> bool {
        self.script.is_some()
    }
    pub fn has_style(&self) -> bool {
        self.template.is_some()
    }
    pub fn set_root(&mut self, root: &'a str) {
        self.root = Cow::Borrowed(root);
    }

    fn convert(ast: &'a parser::ParseResult, root: &'a str) -> Self {
        let mut converter = MakepadConverter::default();
        converter.set_root(root);

        let strategy = ast.strategy();
        // use strategy to convert makepad code
        match strategy {
            parser::Strategy::None => {}
            parser::Strategy::SingleTemplate => {
                let template = handle_template(&converter, ast);
                converter.template.replace(template);
            }
            parser::Strategy::SingleScript => {
                let script = handle_script(ast, true);
                converter.script.replace(script);
            }
            parser::Strategy::SingleStyle => todo!("wait to handle single style strategy"), // Ok(expand_style(s)) , try to find other rsx have use to inject the style or not
            parser::Strategy::TemplateScript => todo!(),
            parser::Strategy::TemplateStyle => {
                // should associate the style with template
                // new a thread to handle style
                let style = handle_style(ast);
                converter.style = style;
                let template = handle_template(&converter, ast);
                converter.template.replace(template);
            }
            parser::Strategy::All => todo!("handle_all wait to build"),
            // parser::Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
            _ => panic!("{}", Errors::UnAcceptConvertRange.to_string()),
        }

        converter
    }

    fn convert_template(
        &self,
        t: &parser::ASTNodes,
        is_ref: bool,
    ) -> Result<Option<MakepadModel>, Errors> {
        handle_tag(t, self.style.as_ref(), is_ref)
    }

    fn convert_script(&self, sc: parser::Script) {
        todo!()
    }

    fn convert_style(s: &parser::ASTNodes) -> Option<ConvertStyle> {
        match s {
            parser::ASTNodes::Style(s) => expand_style(s),
            parser::ASTNodes::Comment(_) => None,
            parser::ASTNodes::Tag(_) => panic!("{}", Errors::UnAcceptConvertRange.to_string()),
        }
    }
}

impl<'a> Display for MakepadConverter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // inner live_design! template
        let t = self
            .template
            .as_ref()
            .unwrap()
            .into_iter()
            .map(|t| t.to_string())
            .collect::<String>();
        f.write_fmt(format_args!(
            "{} = {}{}{}{{ {} }}",
            self.root, "{{", self.root, "}}", &t
        ))

        // f.write_str(&t)
    }
}

/// expand all style sheet
fn handle_style(ast: &parser::ParseResult) -> Option<ConvertStyle> {
    let mut res = HashMap::new();
    for style in ast.style().unwrap() {
        match MakepadConverter::convert_style(style) {
            Some(styles) => res.extend(styles),
            None => return None,
        };
    }
    Some(res)
}

fn handle_template(converter: &MakepadConverter, ast: &parser::ParseResult) -> Vec<MakepadModel> {
    let mut is_ref = true;
    let mut models = Vec::new();
    for template in ast.template().unwrap() {
        if let Ok(Some(model)) = converter.convert_template(template, is_ref) {
            models.push(model);
        }
        is_ref = false;
    }
    models
}

// fn handle_template(
//     ast: &parser::ParseResult,
//     source_name: &str,
//     is_single: bool,
// ) -> Option<String> {
//     let mut f = String::new();
//     let mut ref_tag = true;
//     let templates = ast.template().unwrap();
//     let template_res = templates
//         .into_iter()
//         .map(|t| {
//             let handled_template = MakepadConverter::convert_template(t, ref_tag, is_single);
//             ref_tag = false;
//             return handled_template;
//         })
//         .collect::<Result<Vec<MakepadModel>, Errors>>();
//     match template_res {
//         Ok(t) => {
//             let template_app = format!(
//                 "{} = {{ {} }}{{ \n{}\n }} }}",
//                 source_name,
//                 source_name,
//                 models_to_string(t)
//             );
//             f.push_str(BIND_IMPORT);
//             f.push_str(&template_app);
//             Some(f)
//         }
//         Err(_) => None,
//     }
// }

fn handle_script(ast: &parser::ParseResult, is_single: bool) -> ConvertScript {
    // is_single:
    // true: the script is independent and it will be inject into other rsx , do not need to convert special
    // false: try to convert the script link to makepad script
    // example
    // ```
    // rsx:          let mut counter:u8 = 0;
    // makepad:      pub struct App { #[rust] counter: u8 }
    // ```
    if is_single {
        ConvertScript::Rust(ast.script().unwrap().to_string())
    } else {
        let mut stmts = Vec::new();
        for sc in &ast.script().unwrap().as_origin().stmts {
            match sc {
                syn::Stmt::Local(local) => {
                    stmts.push(handle_variable(local));
                }
                _ => todo!("syn::Stmt need to complate later"),
            }

            // todo!("handle script in rsx");
        }
        // handle_variable()

        // handle_function()
        // sc.stmts
        // todo!("handle script in rsx");
        // Some(())
        ConvertScript::MakepadRS(stmts)
    }
}

fn handle_variable(local: &Local) -> Stmt {
    // get init
    let init = local.init.as_ref();

    let stmt = match &local.pat {
        syn::Pat::Type(t) => {
            // get pat
            let ident = match &*t.pat {
                syn::Pat::Ident(i) => &i.ident,
                _ => todo!(
                    "Handle in pat:Ident! inner! handle variable syn later, see future needed"
                ),
            };
            // get ty
            let ty = &*t.ty;

            let new_stmt: Stmt = parse_quote! {
                #[rust] #ident: #ty
            };
            new_stmt
        }
        syn::Pat::Ident(i) => todo!(),
        _ => todo!("handle variable syn later, see future needed"),
    };

    stmt
}

/// 平展样式
fn expand_style(s: &Box<Style>) -> Option<ConvertStyle> {
    let mut res = HashMap::new();
    // handle props
    if s.has_props() {
        let style_name = s.get_name();
        let props = s.get_props().unwrap();
        match s.get_type() {
            parser::StyleType::Class | parser::StyleType::Id => {
                res.insert(Cow::Borrowed(style_name), Cow::Borrowed(props))
            }
            parser::StyleType::Pseudo => {
                // find the parent and set maybe here need to do something special
                // so write todo to watch
                todo!("style pseudo");
            }
        };
    }
    // handle children
    if s.has_children() {
        for item in s.get_children().unwrap() {
            match MakepadConverter::convert_style(item) {
                Some(styles) => {
                    let _ = res.extend(styles);
                }
                None => {}
            };
        }
    }
    if res.is_empty() {
        return None;
    }
    Some(res)
}

fn handle_tag(
    t: &parser::ASTNodes,
    styles: Option<&ConvertStyle>,
    is_ref: bool,
) -> Result<Option<MakepadModel>, Errors> {
    match t {
        parser::ASTNodes::Tag(t) => {
            // 1. uppercase the first title case of the tag
            // 2. add `<` `>` surround the tag
            // 3. add `{` `}` after the tag
            let tag_name =
                uppercase_title(t.get_name()).expect(&Errors::UppercaseTitleFail.to_string());
            let mut tag_model = MakepadModel::new(&tag_name, is_ref);
            // check props
            if t.has_props() {
                for prop in t.get_props().unwrap() {
                    match PropRole::try_from((tag_name.as_str(), prop)) {
                        Ok(p) => {
                            if p.is_special() {
                                tag_model.set_special(p.to_special());
                            } else if p.is_context() {
                                let _ = p
                                    .to_context()
                                    .into_iter()
                                    .for_each(|x| tag_model.push_context(x));
                            } else {
                                tag_model.push_prop(p);
                            }
                        }
                        Err(e) => panic!("{}", e.to_string()),
                    };
                }
            }
            // have styles
            // true: do not need to associate with styles
            // false: need if style exists
            if styles.is_some() {
                let styles = styles.unwrap();
                // when special and context means link , need to patch
                // check special
                // if tag_model.has_special() {

                // }
                if let Some(links) = tag_model.get_links() {
                    for link in links {
                        // let special = tag_model.get_special().unwrap();
                        // check and then convert
                        // dbg!(&link);
                        if let Some(sheets) = styles.get(&Cow::Borrowed(link.as_str())) {
                            let _ = sheets.iter().try_for_each(|kv| {
                                PropRole::try_from((&tag_name, kv))
                                    .map(|item| tag_model.push_prop(item))
                            });
                        }
                    }
                }
            }
            // children
            if t.has_children() {
                for child in t.get_children().unwrap() {
                    if let Ok(Some(child_template)) = handle_tag(child, styles, false) {
                        let _ = tag_model.push_child(child_template);
                    }
                }
            }

            Ok(Some(tag_model))
        }
        parser::ASTNodes::Comment(c) => Ok(None),
        parser::ASTNodes::Style(s) => panic!("{}", Errors::UnAcceptConvertRange.to_string()),
    }
}

/// Match properties based on the existing components in the current makepad widgets
// fn prop_match(tag: &str, prop: (&PropsKey, &Value)) -> Result<PropRole, Errors> {
//     match tag {
//         "Window" => window(prop.0, prop.1),
//         "Button" => button(prop.0, prop.1),
//         _ => Err(Errors::UnMatchedWidget),
//     }
// }

#[cfg(test)]
mod test_makepad {
    use std::time::Instant;

    use parser::{ParseResult, ParseTarget};

    use super::MakepadConverter;

    #[test]
    fn convert_single_t() {
        // example for: window single button
        // <button id="my_button" text="Hello, World" @clicked="btn_click"></button>

        let input = r#"
        <template>
            <window id="ui" class="my_ui my_ui2">
               <view id="body" class="my_ui2"/>
            </window>
        </template>

        <style>
        #ui{
            padding: 10 16;
            height: 178.9;
            line_spacing: 32.9;
            clip_x: true;
            clip_y: false;
        }
        .my_ui{
            width: Fill;
            background_color: #000;
            background_visible: false;
            align_y: 16;
        }
        .my_ui2{
            margin: 1 3 5 7;
            spacing: 18;
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let convert = MakepadConverter::convert(&ast, "App");
        dbg!(t.elapsed());
        // dbg!(&ast.style());
        dbg!(convert.to_string());
    }

    #[test]
    fn convert_style() {
        let input = r#"
        <style>
        .ui{
            height:100;
            width:120;
            margin:7 10 0 30;
            .button{
                height: 46;
                width: 88.8;
            }
        }
        </style>
        "#;
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let style = ast.style().unwrap();
        style.into_iter().for_each(|x| {
            dbg!(MakepadConverter::convert_style(x));
        });
    }

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
        };
        </script>
        "#;

        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        dbg!(MakepadConverter::convert(&ast, "App"));
    }
}
