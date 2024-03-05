use std::{fmt::Display, sync::mpsc, thread};

use crate::{
    error::{self, Errors},
    target::{parse_script, parse_style, parse_template},
};

use super::{ASTNodes, ParseCore, ParseTarget, Script, Strategy};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParseResult {
    template: Option<Vec<ASTNodes>>,
    style: Option<Vec<ASTNodes>>,
    script: Option<Script>,
}

impl ParseResult {
    pub fn set_template(&mut self, t: Vec<ASTNodes>) {
        let _ = self.template.replace(t);
    }
    pub fn set_script(&mut self, sc: Script) {
        let _ = self.script.replace(sc);
    }
    pub fn set_style(&mut self, s: Vec<ASTNodes>) {
        let _ = self.style.replace(s);
    }
    pub fn template(&self) -> Option<&Vec<ASTNodes>>{
        self.template.as_ref()
    }
    pub fn style(&self) -> Option<&Vec<ASTNodes>>{
        self.style.as_ref()
    }
    pub fn script(&self) -> Option<&Script>{
        self.script.as_ref()
    }
}

impl TryFrom<ParseTarget> for ParseResult {
    type Error = error::Error;

    fn try_from(value: ParseTarget) -> Result<Self, Self::Error> {
        ParseCore::from(value).try_into()
    }
}

impl TryFrom<ParseCore> for ParseResult {
    type Error = error::Error;

    fn try_from(value: ParseCore) -> Result<Self, Self::Error> {
        match value.target_strategy() {
            Strategy::All => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let t_input = value.template().unwrap().clone();
                let s_input = value.style().unwrap().clone();
                let sc_input = value.script().unwrap();

                if let Err(e) = handle_script(&mut result, sc_input) {
                    return Err(e);
                }
                let sender_t = sender.clone();
                // new thread to handle template
                thread::spawn(move || {
                    let res_t = parse_template(&t_input);
                    sender_t
                        .send((res_t, true))
                        .expect("failed to send template");
                });

                thread::spawn(move || {
                    let res_s = parse_style(&s_input);
                    sender.send((res_s, false)).expect("failed to send style");
                });
                for _ in 0..2 {
                    match receiver.recv().expect("failed to receive template") {
                        (Ok(ast), true) => result.set_template(ast),
                        (Ok(ast), false) => result.set_style(ast),
                        (Err(e), true) | (Err(e), false) => return Err(e),
                    };
                }

                Ok(result)
            }
            Strategy::TemplateScript => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let t_input = value.template().unwrap().clone();
                let sc_input = value.script().unwrap();

                if let Err(e) = handle_script(&mut result, sc_input) {
                    return Err(e);
                }

                // new thread to handle template
                thread::spawn(move || {
                    let res_t = parse_template(&t_input);
                    sender.send(res_t).expect("failed to send template");
                });
                match receiver.recv().expect("failed to receive template") {
                    Ok(ast) => {
                        let _ = result.set_template(ast);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(result)
            }
            Strategy::TemplateStyle => {
                let mut result = ParseResult::default();
                // channel
                let (sender, receiver) = mpsc::channel();
                let s_input = value.style().unwrap().clone();
                let t_input = value.template().unwrap();

                if let Err(e) = handle_template(&mut result, t_input) {
                    return Err(e);
                }
                // new thread to handle style
                thread::spawn(move || {
                    let res_s = parse_style(&s_input);
                    sender.send(res_s).expect("failed to send style");
                });
                match receiver.recv().expect("failed to receive style") {
                    Ok(ast) => {
                        let _ = result.set_style(ast);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };

                Ok(result)
            }
            Strategy::SingleTemplate => {
                let mut result = ParseResult::default();
                match handle_template(&mut result, value.template.unwrap().as_str()) {
                    Ok(_) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Strategy::Error(e) => Err(error::Error::convert(Errors::ParseError(e))),
            Strategy::SingleScript => {
                let mut result = ParseResult::default();
                match handle_script(&mut result, value.script.unwrap().as_str()) {
                    Ok(_) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Strategy::SingleStyle => {
                let mut result = ParseResult::default();
                match handle_style(&mut result, value.style.unwrap().as_str()) {
                    Ok(_) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            Strategy::None => Ok(ParseResult::default()),
            _ => Err(error::Error::parse_error(
                "The conversion strategy is temporarily not allowed to be processed",
            )),
        }
    }
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // only need to convert back ParseCore
        write!(f,"{}",ParseTarget::from(ParseCore::from(self.clone())).to_string())
    }
}


fn handle_template(result: &mut ParseResult, input: &str) -> Result<(), error::Error> {
    match parse_template(input) {
        Ok(ast) => {
            result.set_template(ast);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
fn handle_script(result: &mut ParseResult, input: &str) -> Result<(), error::Error> {
    match parse_script(input) {
        Ok(ast) => {
            result.set_script(ast.into());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn handle_style(result: &mut ParseResult, input: &str) -> Result<(), error::Error> {
    match parse_style(input) {
        Ok(ast) => {
            result.set_style(ast);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test_result{
    use std::{fs::File, io::Write, time::Instant};

    use crate::ast::{ParseResult, ParseTarget};

    #[test]
    fn test_result(){
        let input = r#"
        <template>
            <window class="ui">
                <view class="body">
                    /// button componet
                    <button value="Hello world" class="button1" @clicked="handle_actions" />
                    <text-input value="Click to count" class="input1" />
                    <label :value="counter" class="label1" />
                </view>
            </window>
        </template>

        <script>
        let mut counter:usize = 0_usize;

        let mut click = ||{
            counter += 1;
        };
        </script>

        <style>
        .app {
            .ui {
              height: fill;
              width: fill;
              show_bg: true;
              background_color: linear_gradient(180deg, #7, #3);
              .body {
                flow: down;
                spacing: 20;
                align: 0.5 0.5;
                .button1 {
                }
                .input1 {
                  height: 30;
                  width: 100;
                }
                .label1 {
                  color: #ffffff;
                }
              }
            }
          }
        </style>
        "#;
        let t = Instant::now();
        let _ = ParseResult::try_from(ParseTarget::try_from(input).unwrap()).unwrap();
        // cpu:2.2 GHz 四核Intel Core i7
        // 1.332564ms
        // 1.203039ms
        // 1.496007ms
        // 1.229173ms
        // 1.207143ms
        // 1.125941ms
        dbg!(t.elapsed());
        // dbg!(target);
        // let mut f = File::create("/Users/user/Downloads/beyond-framework-main/rsx/parser/t.vue").unwrap();
        // let _ = f.write(result.to_string().as_bytes());
    }
}