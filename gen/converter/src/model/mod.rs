pub mod event;
pub mod prop;
mod script;
mod style;
mod template;

use std::{
    error::Error,
    fs::File,
    io::Read,
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
};

use gen_parser::{ParseResult, ParseTarget, Strategy};
use gen_traits::{event::Event, prop::Prop};
pub use template::TemplateModel;

use self::{
    event::NoEvent,
    prop::{ConvertStyle, NoProps},
    script::ConvertScript,
    style::handle_styles,
};

#[derive(Debug, Clone)]
pub enum ConvertResult {
    Template(Option<TemplateModel>),
    Style(Option<ConvertStyle>),
}

/// # GenUI文件模型
/// 用于表示一个.gen文件，这个文件会被解析为一个模型
/// 每个.gen文件会由解析器解析为一个AST，然后根据AST生成一个模型
/// 在解析器进行解析时，这个文件会被标识一个策略，这个策略会决定这个文件的转换方式
/// ## 策略说明
/// - 如果这个文件只有一个模版，那么这个文件会被标识为SingleTemplate策略
/// - 如果这个文件有模版和脚本，那么这个文件会被标识为TemplateScript策略
/// - ...
/// 通过策略,转换器可以知道如何处理这个文件
#[derive(Debug, Clone, Default)]
pub struct Model {
    /// 模型的唯一标识，通常被认为是该模型的文件路径，根据文件路径可以找到这个模型
    /// 这个字段在模型生成时会被设置
    pub special: String,
    /// 模型的模版部分，即.gen文件的<template>标签包裹的部分
    pub template: Option<TemplateModel>,
    /// 模型的脚本部分，即.gen文件的<script>标签包裹的部分
    pub  script: Option<ConvertScript>,
    /// 模型的样式部分，即.gen文件的<style>标签包裹的部分
    /// 也可以认为是模型的属性部分，在GenUI中并没有属性与样式的区别
    /// ConvertStyle实际上是被平展的样式列表
    pub style: Option<ConvertStyle>,
    /// 模型是否需要被编译
    /// 在项目中可能存在一个文件被编写，但没有在项目中使用到
    /// 表现为这个文件没有使用Rust的use语句进行引入
    pub compile: bool,
}

impl Model {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        match file_data(path) {
            Ok(input) => {
                let mut model = Model::default();
                let ast =
                    ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
                let _ = Model::convert(&mut model, ast, path);
                Ok(model)
            }
            Err(e) => Err(e),
        }
    }
    pub fn set_template(&mut self, template: TemplateModel) -> () {
        let _ = self.template.replace(template);
    }
    pub fn set_script(&mut self, script: ConvertScript) -> () {
        let _ = self.script.replace(script);
    }
    pub fn set_style(&mut self, style: ConvertStyle) -> () {
        let _ = self.style.replace(style);
    }
    pub fn set_compile(&mut self) {
        self.compile = true;
    }
    pub fn get_compile(&self) -> bool {
        self.compile
    }
    pub fn get_template(&self) -> Option<&TemplateModel> {
        self.template.as_ref()
    }
    pub fn get_template_mut(&mut self) -> Option<&mut TemplateModel> {
        self.template.as_mut()
    }
    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }
    pub fn get_styles(&self) -> Option<&ConvertStyle> {
        self.style.as_ref()
    }
    pub fn get_styles_mut(&mut self) -> Option<&mut ConvertStyle> {
        self.style.as_mut()
    }
    pub fn has_styles(&self) -> bool {
        self.style.is_some()
    }
    pub fn get_script(&self) -> Option<&ConvertScript> {
        self.script.as_ref()
    }
    /// 通过parser层解析的结果和文件路径生成converter层模型
    /// 这一层只需要处理template和style部分，script不变
    fn convert(model: &mut Model, ast: ParseResult, path: &Path) -> () {
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
                let template_sender = sender.clone();
                let style_sender = sender.clone();
                let template = ast.template().unwrap()[0].clone();
                let styles = ast.style().unwrap().clone();
                let script = ast.script().unwrap().clone();
                model.set_script(script);
                let _ = thread::spawn(move || {
                    let convert_res = TemplateModel::convert(&template, true);
                    template_sender
                        .send(ConvertResult::Template(convert_res))
                        .expect("send template error");
                });
                let _ = thread::spawn(move || {
                    let convert_res = handle_styles(&styles);
                    style_sender
                        .send(ConvertResult::Style(convert_res))
                        .expect("send style error");
                });

                for _ in 0..2 {
                    match receiver
                        .recv()
                        .expect("gen_converter: receive failed when convert!")
                    {
                        ConvertResult::Template(t) => {
                            if t.is_some() {
                                model.set_template(t.unwrap());
                            } else {
                                panic!("template cannot be none in Strategy::All")
                            }
                        }
                        ConvertResult::Style(s) => {
                            if s.is_some() {
                                model.set_style(s.unwrap());
                            } else {
                                panic!("style cannot be none in Strategy::All")
                            }
                        }
                    }
                }
            }
            // Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
            _ => panic!("Invalid strategy!"),
        }
    }

    pub fn set_special(&mut self, special: &str) -> () {
        if self.special.is_empty() {
            self.special = special.to_string();
        } else {
            panic!("special is already set");
        }
    }
}

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
