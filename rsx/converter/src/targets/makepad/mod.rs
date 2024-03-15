pub mod constants;
pub mod model;
mod prop;
pub mod result;
mod script;
mod style;
pub mod value;
mod widget;

pub use prop::*;
use quote::{quote, ToTokens};
pub use script::*;
pub use style::*;
use syn::{parse_quote, token::Mut, Local, LocalInit, Stmt, Type};
pub use widget::*;

use std::{borrow::Cow, collections::HashMap, fmt::Display};

use parser::{ASTNodes, PropsKey, Style, Tag, Value, HOLDER_END};

use crate::{error::Errors, traits::Visitor, utils::alphabetic::uppercase_title};

use self::{
    constants::{APP_MAIN, BIND_IMPORT, LIVE_REGISTER},
    model::MakepadModel,
    value::MakepadPropValue,
};

type ConvertStyle<'a> = HashMap<Cow<'a, str>, Cow<'a, HashMap<PropsKey, Value>>>;
/// `(tag_name, (prop_name, prop_value))`
pub type BindProp = (String, (String, MakepadPropValue));
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MakepadConverter<'a> {
    root: Cow<'a, str>,
    // single model
    template: Option<MakepadModel>,
    script: Option<ConvertScript>,
    style: Option<ConvertStyle<'a>>,
    widget_ref: Option<Cow<'a, str>>,
    bind_props: Option<Vec<BindProp>>,
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

    pub fn set_widget_ref(&mut self) -> () {
        if let Some(t) = &self.template {
            match t.get_special() {
                Some(ref_ui_name) => {
                    let _ = self.widget_ref.replace(Cow::Owned(ref_ui_name.to_string()));
                }
                None => todo!("set default special name as widget ref"),
            }
        }
    }

    fn convert(ast: &'a parser::ParseResult, root: &'a str) -> Self {
        let mut converter = MakepadConverter::default();
        converter.set_root(root);

        let strategy = ast.strategy();
        // use strategy to convert makepad code
        match strategy {
            parser::Strategy::None => {}
            parser::Strategy::SingleTemplate => {
                converter.convert_template(&ast.template().unwrap()[0])
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
                converter.convert_template(&ast.template().unwrap()[0]);

                converter.set_widget_ref();
            }
            parser::Strategy::All => {
                // should associate the style with template
                // new a thread to handle style
                let script = handle_script(ast, false);
                converter.script.replace(script);

                let style = handle_style(ast);
                converter.style = style;
                // let template = handle_template(&converter, ast);
                converter.convert_template(&ast.template().unwrap()[0]);
                // converter.template.replace(template);
                converter.set_widget_ref();
            }
            // parser::Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
            _ => panic!("{}", Errors::UnAcceptConvertRange.to_string()),
        }

        converter
    }

    fn convert_template(&mut self, t: &ASTNodes) -> () {
        match t {
            ASTNodes::Tag(t) => {
                let (model, binds) = handle_tag(*&t, self.style.as_ref(), true);
                let _ = self.template.replace(model);
                let _ = self.bind_props.replace(binds);
            }
            ASTNodes::Comment(_) => todo!(),
            ASTNodes::Style(_) => todo!(),
        }
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
        let t = self.template.as_ref().unwrap().to_string();
        let t_fmt = format!("{} = {}{}{}{{ {} }}", self.root, "{{", self.root, "}}", &t);
        // write live_design code
        let _ = f.write_fmt(format_args!("{}\n{}\n{}", BIND_IMPORT, t_fmt, HOLDER_END));

        // dbg!(self.script.as_ref().unwrap().to_string());

        match &self.widget_ref {
            Some(name) => {
                let _ = f.write_fmt(format_args!(
                    "\n#[derive(Live, LiveHook)]\npub struct App {{ #[live] {}: WidgetRef",
                    name,
                ));
                if self.has_script() {
                    let sc = self.script.as_ref().unwrap();
                    match sc.get_makepad_vars() {
                        Some(vars) => {
                            // get bind props
                            let binds = self.bind_props.as_ref().unwrap();
                            let bind_instance = vars_to_string(vars, binds);
                        }
                        None => {}
                    }
                    // let _ = f.write_fmt(format_args!(
                    //     ", {} }}",
                    //     self.script.as_ref().unwrap().to_string()
                    // ));
                } else {
                    let _ = f.write_str(HOLDER_END);
                }
            }
            None => {}
        }

        let _ = f.write_fmt(format_args!(
            "\nimpl LiveRegister for {} {{ {} }}",
            self.root, LIVE_REGISTER
        ));
        f.write_fmt(format_args!(
            "impl AppMain for {} {{ {} {{ {} }} }}",
            self.root, APP_MAIN, "self.ui.handle_event(cx, event, &mut Scope::empty());"
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

fn handle_script(ast: &parser::ParseResult, is_single: bool) -> ConvertScript {
    // is_single:
    // true: the script is independent and it will be inject into other rsx , do not need to convert special
    // false: try to convert the script link to makepad script
    // example
    // ```
    // rsx:          let mut counter: u8 = 0;
    // makepad:      #[rust] counter: u8
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

fn handle_variable(local: &Local) -> ScriptNode {
    // get init
    let init = local.init.clone();
    // dbg!(&local);
    let stmt = match &local.pat {
        syn::Pat::Type(t) => {
            // get pat
            let (name, is_mut) = match &*t.pat {
                syn::Pat::Ident(i) => (i.ident.to_string(), i.mutability.is_some()),
                _ => panic!("unexpect pat type in this script"),
            };
            // get ty
            let ty = &*t.ty;
            NodeVariable::new_unwrap(name, ty.clone(), init, is_mut)
        }
        syn::Pat::Ident(i) => {
            let name = i.ident.to_string();
            let is_mut = i.mutability.is_some();
            let (ty, init) = parse_init_type(init);
            NodeVariable::new_unwrap(name, ty, init, is_mut)
        }
        _ => todo!("handle variable syn later, see future needed"),
    };

    ScriptNode::Variable(stmt)
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

/// acturally if the handle_tag() function can run
/// it must have ConvertScript
fn handle_tag(
    t: &Tag,
    styles: Option<&ConvertStyle>,
    is_ref: bool,
) -> (MakepadModel, Vec<BindProp>) {
    // 1. uppercase the first title case of the tag
    // if can not upper - panic!
    let tag_name = uppercase_title(t.get_name()).unwrap();
    // 2. add `<` `>` surround the tag
    // 3. add `{` `}` after the tag
    let mut tag_model = MakepadModel::new(&tag_name, is_ref);
    let mut binds = Vec::new();
    // check props
    if t.has_props() {
        for prop in t.get_props().unwrap() {
            match PropRole::try_from((tag_name.as_str(), prop)) {
                Ok(p) => {
                    // dbg!(&p);
                    match p {
                        PropRole::Normal(_, _) => tag_model.push_prop(p),
                        PropRole::Bind(k, v) => {
                            binds.push((tag_name.clone(), (k, v)));
                            // if is bind need to get real value from script
                            // should use k as the judge condition
                            // match script.unwrap().get_makepad_vars() {
                            //     Some(sc) => {
                            //         let var_name = v.get_bind_key().to_string();
                            //         let mut is_found = false;
                            //         for var in sc {
                            //             if var.get_name() == &var_name {
                            //                 // do value check for data
                            //                 // dbg!(&v);
                            //                 let _ = v.set_bind_value(
                            //                     PropRole::try_from((&tag_name, (&k, var)))
                            //                         .unwrap()
                            //                         .into(),
                            //                 );
                            //                 // dbg!(&p);
                            //                 tag_model.push_prop(PropRole::bind(&k, v));
                            //                 is_found = true;
                            //                 break;
                            //             }
                            //         }
                            //         if !is_found {
                            //             panic!(
                            //                 "Could not find bind key:{} in script",
                            //                 &var_name
                            //             );
                            //         }
                            //     }
                            //     None => panic!(
                            //         "prop: {} is a bind prop, which lack of binding value",
                            //         k
                            //     ),
                            // }
                            // dbg!(&tag_model);
                        }
                        PropRole::Function => todo!("function do!!!!"),
                        PropRole::Context(c) => {
                            c.into_iter().for_each(|x| tag_model.push_context(x));
                        }
                        PropRole::Special(s) => tag_model.set_special(s),
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
                    let (child_model, child_binds) = handle_tag(*&child, styles, false);
                    tag_model.push_child(child_model);
                    binds.extend(child_binds);
                }
                ASTNodes::Comment(_) => (),
                ASTNodes::Style(_) => panic!("{}", "cannot write styles in template node"),
            }
        }
    }

    (tag_model, binds)
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
            flow: Overlay;
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
