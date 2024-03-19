pub mod action;
pub mod constants;
pub mod model;
mod prop;
pub mod result;
mod script;
mod style;
pub mod value;
mod widget;

pub use prop::*;

pub use script::*;
pub use style::*;
use syn::{Expr, Local, LocalInit};
pub use widget::*;

use std::{borrow::Cow, collections::HashMap, fmt::Display};

use parser::{ASTNodes, PropsKey, Tag, Value, HOLDER_END};

use crate::{
    error::Errors, targets::makepad::action::MakepadAction, utils::alphabetic::uppercase_title,
};

use self::{
    constants::{APP_MAIN, BIND_IMPORT, LIVE_REGISTER},
    model::MakepadModel,
    value::MakepadPropValue,
};

pub type ConvertStyle<'a> = HashMap<Cow<'a, str>, Cow<'a, HashMap<PropsKey, Value>>>;
/// `(tag_name, id, (prop_name, prop_value))`
pub type BindProp = (String, String, (String, MakepadPropValue));
/// `(tag_name, id, (action_name, action_var_name))`
pub type BindAction = (String, String, (String, String));
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MakepadConverter<'a> {
    root: Cow<'a, str>,
    // single model
    template: Option<MakepadModel>,
    script: Option<ConvertScript>,
    style: Option<ConvertStyle<'a>>,
    widget_ref: Option<Cow<'a, str>>,
    bind_props: Option<Vec<BindProp>>,
    bind_actions: Option<Vec<BindAction>>,
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
                let (model, binds, actions) = handle_tag(*&t, self.style.as_ref(), true);
                let _ = self.template.replace(model);
                let _ = self.bind_props.replace(binds);
                let _ = self.bind_actions.replace(actions);
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
        let mut start_up = false;
        let mut event_match = false;
        match &self.widget_ref {
            Some(name) => {
                let _ = f.write_fmt(format_args!(
                    "\n#[derive(Live, LiveHook)]\npub struct App {{ #[live] {}: WidgetRef, #[rust] instance: Instance,}}\n",
                    name,
                ));
                if self.has_script() {
                    start_up = true;
                    let sc = self.script.as_ref().unwrap();
                    // match sc.get_makepad_vars() {
                    //     Some(vars) => {
                    //         // get bind props
                    //         let binds = self.bind_props.as_ref().unwrap();
                    //         let name = self.root.to_string();
                    //         let bind_instance = vars_to_string(name, vars, binds);
                    //         let _ = f.write_str(&bind_instance);
                    //     }
                    //     None => {}
                    // }
                    // let _ = f.write_fmt(format_args!(
                    //     ", {} }}",
                    //     self.script.as_ref().unwrap().to_string()
                    // ));

                    let (m_vars, m_fns) = sc.get_makepad_var_fn();
                    match m_vars {
                        Some(vars) => {
                            // get bind props
                            let binds = self.bind_props.as_ref().unwrap();
                            let name = self.root.to_string();
                            let bind_instance = vars_to_string(name, vars, binds);
                            let _ = f.write_str(&bind_instance);
                        }
                        None => {}
                    }
                    // dbg!(&m_fns);

                    match m_fns {
                        Some(fns) => {
                            event_match = true;
                            let mut fns = fns.into_iter().map(|item| item.clone()).collect();
                            let name = self.root.to_string();
                            let actions = self.bind_actions.as_ref().unwrap();
                            let binds = self.bind_props.as_ref();
                            let _ =
                                f.write_str(fns_to_string(name, &mut fns, actions, binds).as_str());
                        }
                        None => {}
                    }
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
        let _ = f.write_fmt(format_args!(
            "impl AppMain for {} {{ {} {{ ",
            self.root, APP_MAIN
        ));
        if start_up {
            let _ = f.write_str("match event{ Event::Startup => self.start_up(cx), _ =>() };");
        }
        if event_match {
            let _ = f.write_str("self.match_event(cx, event);");
        }
        let _ = f.write_str("self.ui.handle_event(cx, event, &mut Scope::empty());} }");
        f.write_fmt(format_args!("app_main!({});", self.root))
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

fn is_closure(init: Option<&LocalInit>) -> bool {
    let expr = *init.unwrap().expr.clone();
    matches!(expr, Expr::Closure(_))
}

fn handle_variable(local: &Local) -> ScriptNode {
    // get init
    let init = local.init.clone();
    match &local.pat {
        syn::Pat::Type(t) => {
            // get pat
            let (name, is_mut) = match &*t.pat {
                syn::Pat::Ident(i) => (i.ident.to_string(), i.mutability.is_some()),
                _ => panic!("unexpect pat type in this script"),
            };
            // get ty
            let ty = &*t.ty;

            ScriptNode::Variable(NodeVariable::new_unwrap(name, ty.clone(), init, is_mut))
        }
        syn::Pat::Ident(i) => {
            let name = i.ident.to_string();
            let is_mut = i.mutability.is_some();
            if is_closure(init.as_ref()) {
                if !is_mut {
                    panic!("closure must be mutable")
                }
                // handle closure -> function
                ScriptNode::Function(MakepadAction::new(&name, *init.unwrap().expr, is_mut))
            } else {
                let is_mut = i.mutability.is_some();
                let (ty, init) = parse_init_type(init);
                ScriptNode::Variable(NodeVariable::new_unwrap(name, ty, init, is_mut))
            }
        }
        _ => todo!("handle variable syn later, see future needed"),
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
    let tag_name = uppercase_title(t.get_name()).unwrap();
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
                None => panic!("the widget which has binds need to add special id"),
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
