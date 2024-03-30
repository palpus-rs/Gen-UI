pub mod action;
pub mod prop;
pub mod event;
mod script;
mod style;
mod template;

use std::{
    borrow::Cow,
    error::Error,
    ffi::{OsStr, OsString},
    fs::{read_to_string, DirEntry, File},
    io::Read,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};

use action::ModelAction;
use gen_parser::{ParseResult, ParseTarget, Strategy};
use prop::ConvertProp;
pub use template::TemplateModel;

use self::{
    action::Action, prop::{ConvertStyle, Props}, script::ConvertScript, style::handle_styles,
    template::handle_template,
};

/// # GenUI文件模型
/// 用于表示一个.gen文件，这个文件会被解析为一个模型
/// 每个.gen文件会由解析器解析为一个AST，然后根据AST生成一个模型
/// 在解析器进行解析时，这个文件会被标识一个策略，这个策略会决定这个文件的转换方式
/// ## 策略说明
/// - 如果这个文件只有一个模版，那么这个文件会被标识为SingleTemplate策略
/// - 如果这个文件有模版和脚本，那么这个文件会被标识为TemplateScript策略
/// - ...
/// 通过策略,转换器可以知道如何处理这个文件
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model {
    /// 模型的唯一标识，通常被认为是该模型的文件路径，根据文件路径可以找到这个模型
    /// 这个字段在模型生成时会被设置
    special: String,
    /// 模型的模版部分，即.gen文件的<template>标签包裹的部分
    template: Option<TemplateModel>,
    /// 模型的脚本部分，即.gen文件的<script>标签包裹的部分
    script: Option<ConvertScript>,
    /// 模型的样式部分，即.gen文件的<style>标签包裹的部分
    /// 也可以认为是模型的属性部分，在GenUI中并没有属性与样式的区别
    /// ConvertStyle实际上是被平展的样式列表
    style: Option<ConvertStyle>,
    /// 模型是否需要被编译
    /// 在项目中可能存在一个文件被编写，但没有在项目中使用到
    /// 表现为这个文件没有使用Rust的use语句进行引入
    complie: bool,
}

impl Model {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        match file_data(path) {
            Ok(input) => {
                let ast =
                    ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
                Ok(Model::convert(ast, path))
            }
            Err(e) => Err(e),
        }
    }
    fn convert(ast: ParseResult, path: &Path)->Self{
        let mut model = Model::default();
        let _ = model.set_special(path.to_str().unwrap());
        // get strategy
        match &ast.strategy() {
            Strategy::None => {}
            Strategy::SingleTemplate => todo!(),
            Strategy::SingleScript => todo!(),
            Strategy::SingleStyle => todo!("wait to handle single style strategy"), // Ok(expand_style(s)) , try to find other rsx have use to inject the style or not
            Strategy::TemplateScript => todo!(),
            Strategy::TemplateStyle => todo!(),
            Strategy::All => {
                let (sender, receiver) = mpsc::channel();
                let style_sender = sender.clone();
                let script_sender = sender.clone();
                let template_sender = sender;
                let styles = ast.style().unwrap().clone();
                let template = ast.template().unwrap()[0].clone();
                let _ = thread::spawn(move || {
                    let styles = handle_styles(&styles);
                   style_sender.send((1_u8, styles)).expect("send style error");
                });

                // let _ = thread::spawn(move || {
                //     let templates = handle_template(&template);
                //     template_sender.send((10_u8, templates)).expect("send style error");
                // });

                // match receiver.recv().expect("receive style error") {
                //     (handled_styles, true) | (handled_styles, true) => {
                //         model.style = handled_styles
                //     }
                //     (None, false) | (Some(_), false) => todo!(),
                // }
            }
            // Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
            _ => panic!("Invalid strategy!"),
        }

        model
    }

    pub fn set_special(&mut self, special: &str) -> () {
        if self.special.is_empty() {
            self.special = special.to_string();
        }
        panic!("special is already set");
    }
}

// #[derive(Debug, Clone, PartialEq, Default)]
// pub struct Model {
//     /// file path of the model also the model struct name
//     special: String,
//     // single model from template
//     template: Option<TemplateModel>,
//     script: Option<ConvertScript>,
//     // styles from style
//     styles: Option<ConvertStyle>,
//     widget_ref: Option<String>,
//     // props from template
//     props: Props,
//     // actions from template
//     actions: Option<Vec<Action>>,
// }



// impl Model {
//     pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
//         match file_data(path) {
//             Ok(input) => {
//                 let ast =
//                     ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
//                 Ok(Model::convert(ast, path))
//             }
//             Err(e) => Err(e),
//         }

//         // Model::convert(ast, file_name.to_str().unwrap(), file_path)
//     }
//     pub fn set_special(&mut self, special: &str) -> () {
//         self.special = special.to_string();
//     }
//     pub fn has_template(&self) -> bool {
//         self.template.is_some()
//     }
//     pub fn has_script(&self) -> bool {
//         self.script.is_some()
//     }
//     pub fn has_style(&self) -> bool {
//         self.template.is_some()
//     }

//     fn convert(ast: ParseResult, special: &str) -> Self {
//         let mut model = Model::default();
//         let _ = model.set_special(special);
//         // get strategy
//         match &ast.strategy() {
//             Strategy::None => {}
//             Strategy::SingleTemplate => todo!(),
//             Strategy::SingleScript => todo!(),
//             Strategy::SingleStyle => todo!("wait to handle single style strategy"), // Ok(expand_style(s)) , try to find other rsx have use to inject the style or not
//             Strategy::TemplateScript => todo!(),
//             Strategy::TemplateStyle => todo!(),
//             Strategy::All => {
//                 let (sender, receiver) = mpsc::channel();
//                 let style_sender = sender.clone();
//                 let styles = ast.style().unwrap().clone();
//                 let template = ast.template().unwrap()[0].clone();
//                 let _ = thread::spawn(move || {
//                     let styles = handle_styles(&styles);
//                     style_sender.send((styles, true)).expect("send style error");
//                 });

//                 let _ = thread::spawn(move || {
//                     let model_props = handle_template(&template);
//                     style_sender.send((model_props, true)).expect("send style error");
//                 });

//                 match receiver.recv().expect("receive style error") {
//                     (handled_styles, true) | (handled_styles, true) => {
//                         model.styles = handled_styles
//                     }
//                     (None, false) | (Some(_), false) => todo!(),
//                 }
//             }
//             // Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
//             _ => panic!("Invalid strategy!"),
//         }

//         model
//     }
// }

fn file_data<P>(path: P) -> Result<String, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer);
            Ok(buffer)
        }
        Err(e) => Err(Box::new(e)),
    }
}
