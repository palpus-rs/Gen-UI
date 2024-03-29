// #[derive(Debug, Clone, PartialEq)]
// pub struct DataSource {
//     file_name: String,

use std::fmt::Display;

use parser::ParseResult;

use crate::utils::alphabetic::snake_to_camel;

use super::MakepadConverter;

#[derive(Debug, Clone, PartialEq)]
pub struct MakepadConvertResult {
    is_root: bool,
    // data_source: DataSource,
    file_name: String,
    /// after convert to Makepad Code
    code: Option<String>,
    // true: after code be converted to Makepad Code
    // is_update: bool,
    // true: if code need to be refreshed
    // false: code do not need to be refreshed
    // is_refresh:bool
}

#[allow(dead_code)]
impl MakepadConvertResult {
    /// use MakepadConverter to convert rsx to Makepad Code
    /// it will build live_design! part and some impls for Target
    pub fn new(is_root: bool, file_name: &str, ast: ParseResult) -> MakepadConvertResult {
        let code = MakepadConverter::convert(&ast, &snake_to_camel(file_name)).to_string();

        MakepadConvertResult {
            is_root,
            file_name: file_name.to_string(),
            code: Some(code),
            // is_update:false,
            // is_refresh: true,
        }
    }

    pub fn update(&mut self, ast: ParseResult) -> () {
        let code = MakepadConverter::convert(&ast, &snake_to_camel(&self.file_name)).to_string();
        self.code.replace(code);
    }
}

impl Display for MakepadConvertResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Some(c) => {
                let _ = f.write_str(c);
                if self.is_root {
                    let _ = f.write_fmt(format_args!(
                        "\napp_main!({});",
                        snake_to_camel(&self.file_name)
                    ));
                }
            }
            None => {}
        };

        write!(f, "{}", "\n")
    }
}

#[cfg(test)]
mod test_result_mk {
    use std::{fs::File, io::Write, time::Instant};

    use parser::{ParseResult, ParseTarget};

    use crate::{
        targets::makepad::{result::MakepadConvertResult, MakepadConverter},
        utils::alphabetic::snake_to_camel,
    };

    #[test]
    fn test_main() {
        let input = r#"
        <template>
            <window id="ui">
                
            </window>
        </template>
        <style>
        #ui{
            flow: RightWrap;
            
            cursor:Hidden;
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let result = MakepadConvertResult::new(true, "App", ast);
        dbg!(t.elapsed());
        //"/Users/user/Downloads/beyond-framework-main/rsx/converter/wiki/convert.rs"
        //E:/Rust/try/makepad/rsx/converter/wiki/convert.rs
        let mut f = File::create(
            "/Users/user/Downloads/beyond-framework-main/rsx/converter/wiki/convert.rs",
        )
        .unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }

    #[test]
    fn test_simple() {
        let input = r#"
        <template>
            <window id="ui" background_visible="true">
                <view id="body" :spacing="view_space" :flow="view_flow">
                    <button id="btn1" :text="btn_text" @clicked="change_text"></button>
                    <label id="t_label" :text="label_text" :font_size="label_size" />
                </view>
            </window>
        </template>
        
        <script>
        let view_space:f64 = 20;
        let mut view_flow = String::from("Down");
        let mut label_text = String::from("this is a Hello, World!! emoji failed");
        let label_size = 24.0;
        let btn_text = String::from("Click Me");
        
        let mut change_text = || {
            label_text = String::from("I have been clicked!");
        };
        </script>
        
        <style>
        #ui{
            width: Fill;
            height: Fill;
            background_color: #96CEF8;
            #body{
               align: 0.5;
               #t_label{
                    brightness: 1.1;
                    color: #fff;
                    wrap: Word;
                    font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
               }
            }
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        // let result = MakepadConverter::convert(&ast, "App");
        let result = MakepadConvertResult::new(true, "my_app", ast);
        dbg!(t.elapsed());
        // dbg!(result.to_string());
        // E:/Rust/learn/makepad/makepad-rik/examples/simple/src/app.rs
        // /Users/user/Downloads/makepad-rik/examples/single/window_s/src/app.rs
        // E:/Rust/try/makepad/rsx/converter/wiki/convert.rs
        let mut f = File::create(
            "/Users/user/Workspace/others/beyond-framework/rsx/converter/wiki/convert.rs",
        )
        .unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }

    #[test]
    fn test_widget() {
        let input = r#"
        <template>
            <component inherits="view" :props="my_props">
                <label id="first_lb" class="t_label" font_size="32" :text="my_props.label1"/>
                <label id="second_lb" class="t_label" :font_size="fs"  text="label 2"/>
                <button id="bb" text="text btn" @clicked="btn_click" />
            </component>
        </template>
        
        <script>
        #[derive(Default)]
        pub struct MyProps{
            pub label1: String
        }

        let my_props = MyProps::default();
        
        let fs: f64 = 18.0;
        let mut btn_click = ||{
            log!("Button bb Clicked");
        };
        
        </script>
        
        <style>
        .t_label{
            brightness: 1.1;
            color: #fff;
            wrap: Word;
            font: "crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf";
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let name = snake_to_camel("easy_widget");
        let result = MakepadConverter::convert(&ast, &name);
        // let result = MakepadConvertResult::new(true, "easy_widget", ast);
        dbg!(t.elapsed());

        // dbg!(result.to_string());
        // /Users/user/Workspace/others/beyond-framework/rsx/converter/wiki/widget2.rs
        // E:/Rust/try/makepad/Gen-UI/rsx/converter/wiki/widget2.rs
        let mut f = File::create(
            "/Users/user/Workspace/others/beyond-framework/rsx/converter/wiki/widget2.rs",
        )
        .unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }
}
