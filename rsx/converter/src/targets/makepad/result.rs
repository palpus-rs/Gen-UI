// #[derive(Debug, Clone, PartialEq)]
// pub struct DataSource {
//     file_name: String,

use std::fmt::Display;

use parser::{ParseResult, HOLDER_END};

use crate::targets::makepad::constants::BIND_IMPORT;

use super::MakepadConverter;

// }
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
        let code = MakepadConverter::convert(&ast, "App").to_string();

        MakepadConvertResult {
            is_root,
            file_name: file_name.to_string(),
            code: Some(code),
            // is_update:false,
            // is_refresh: true,
        }
    }

    pub fn update(&mut self, ast: ParseResult) -> () {
        let code = MakepadConverter::convert(&ast, "App").to_string();
        self.code.replace(code);
    }
}

impl Display for MakepadConvertResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.code {
            Some(c) => {
                let _ = f.write_str(c);
                if self.is_root {
                    let _ = f.write_fmt(format_args!("\napp_main!({});", self.file_name));
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

    use crate::targets::makepad::{result::MakepadConvertResult, MakepadConverter};

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
        //let no_init:&str;
        // let win_pad = "10";
        // let click_me = ||->(){
        //     println!("clicked!");
        // };
        //:margin="view_margin"
        // background_visible: true;
        // width: Fill;
        // height: Fill;
        // background_color: #7733ff;
        // spacing: 20;
        // align: 0.5 0.5;
        let input = r#"
        <template>
            <window id="ui"  :flow="f1" :clip_x="cx" :cursor="c_hand">
                <view id="body" :height="view_hei" :line_spacing="line_s"/>
            </window>
        </template>
        
        <script>
        // use rsx::on_start_up;
        // let win_pad = "10 16";
        // let win_pad = 17; 
        // let win_pad = String::from("89 16"); ✅
        // let win_pad:String = String::from("32 24"); ✅
        // let win_pad:&str = "10 16"; ✅ 👍
        // let win_pad = "10 16"; ✅ 👍
        // let win_pad:String = String::from("10 16");
        let mut view_hei:String = String::from("Fit");
        // let view_width = "All";
        let f1 = "Down";
        // let line_s:f64 = 20;
        // let cx = true;
        // let c_hand:String = "Hand".to_string();

        // on_start_up(||{
        //     view_hei = "Fill";
        // })
        </script>
        
        <style>
        #ui{
            width: All;
            #body{
               
            }
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let result = MakepadConverter::convert(&ast, "App");
        // let result = MakepadConvertResult::new(true, "App", ast);
        dbg!(t.elapsed());
        // dbg!(result.to_string());
        //"/Users/user/Workspace/others/beyond-framework/rsx/converter/wiki/convert.rs"
        // /Users/user/Downloads/makepad-rik/examples/single/window_s/src/app.rs
        // E:/Rust/try/makepad/rsx/converter/wiki/convert.rs
        let mut f = File::create("E:/Rust/try/makepad/rsx/converter/wiki/convert.rs").unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }
}
