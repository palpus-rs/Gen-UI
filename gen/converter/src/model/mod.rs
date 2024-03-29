pub mod action;
pub mod prop;
mod script;
mod style;
mod template;

use std::{
    borrow::Cow,
    error::Error,
    ffi::OsString,
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
    action::Action, prop::Props, script::ConvertScript, style::handle_styles,
    template::handle_template,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model {
    /// file path of the model also the model struct name
    special: String,
    // single model from template
    template: Option<TemplateModel>,
    script: Option<ConvertScript>,
    // styles from style
    styles: Option<ConvertProp>,
    widget_ref: Option<String>,
    // props from template
    props: Props,
    // actions from template
    actions: Option<Vec<Action>>,
}

impl Model {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        match file_data(path) {
            Ok(input) => {
                let ast =
                    ParseResult::try_from(ParseTarget::try_from(input.as_str()).unwrap()).unwrap();
                Ok(Model::convert(ast, path))
            }
            Err(e) => Err(e),
        }

        // Model::convert(ast, file_name.to_str().unwrap(), file_path)
    }
    pub fn set_special(&mut self, special: &str) -> () {
        self.special = special.to_string();
    }
    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }
    pub fn has_script(&self) -> bool {
        self.script.is_some()
    }
    pub fn has_style(&self) -> bool {
        self.template.is_some()
    }

    fn convert(ast: ParseResult, special: &str) -> Self {
        let mut model = Model::default();
        let _ = model.set_special(special);
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
                let styles = ast.style().unwrap().clone();
                let template = ast.template().unwrap()[0].clone();
                let _ = thread::spawn(move || {
                    let styles = handle_styles(&styles);
                    style_sender.send((styles, true)).expect("send style error");
                });

                let _ = thread::spawn(move || {
                    let styles = handle_template(&template);
                    style_sender.send((styles, true)).expect("send style error");
                });

                match receiver.recv().expect("receive style error") {
                    (handled_styles, true) | (handled_styles, true) => {
                        model.styles = handled_styles
                    }
                    (None, false) | (Some(_), false) => todo!(),
                }
            }
            // Strategy::Error(_) => Err(Errors::UnAcceptConvertRange),
            _ => panic!("Invalid strategy!"),
        }

        model
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
