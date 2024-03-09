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
        // write live_design code
        let _ = f.write_fmt(format_args!(
            "{}\n{}\n{}",
            BIND_IMPORT,
            self.code.as_ref().unwrap(),
            HOLDER_END
        ));

        if self.is_root {
            let _ = f.write_fmt(format_args!("\napp_main!({});", self.file_name));
        }
        write!(f, "{}", "\n")
    }
}

#[cfg(test)]
mod test_result_mk {
    use std::{fs::File, io::Write, time::Instant};

    use parser::{ParseResult, ParseTarget};

    use crate::targets::makepad::result::MakepadConvertResult;

    #[test]
    fn test_main() {
        let input = r#"
        <template>
            <window id="ui" class="my_ui my_ui2">
               <view id="body" class="my_ui2"/>
            </window>
        </template>
        <style>
        #ui{
            align_x: 16;
        }
        .my_ui{
            width: Fill;
            background_color: #000;
            background_visible: false;
        }
        .my_ui2{
            margin: 1 3 5 7;
            spacing: 18;
        }
        </style>
        "#;
        let t = Instant::now();
        let ast = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        let result = MakepadConvertResult::new(true, "App", ast);
        dbg!(t.elapsed());
        //"/Users/user/Downloads/beyond-framework-main/rsx/converter/wiki/convert.rs"
        let mut f = File::create("E:/Rust/try/makepad/rsx/converter/wiki/convert.rs").unwrap();
        let _ = f.write(result.to_string().as_bytes());
    }
}
