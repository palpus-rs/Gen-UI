#[allow(unused_imports)]
use std::{
    default,
    path::{Path, PathBuf},
};

use gen_utils::{
    common::msg::COMPILER_SERVICE,
    compiler::{Builder, CompilerImpl, FromConfig},
};

use crate::{error, MakepadBuilder, CONF, TARGET};
use serde::{Deserialize, Serialize};

use super::error_and_exit;

// use makepad_gen_plugin::compiler::builder::CompilerBuilder as MakepadBuilder;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetCompiler {
    Slint,
    Dioxus,
    #[default]
    Makepad,
}

pub struct Target;

impl Target {
    /// ## Get the current target compiler from config file
    /// 1. get the environment variable `GENUI_TARGET` if not exist, use makepad as default
    /// 2. get the config file `gen.toml`
    pub fn conf() -> Option<Box<dyn CompilerImpl>> {
        let mut target = std::env::var("GENUI_TARGET").unwrap_or("makepad".to_string());

        match CONF.lock().unwrap().as_ref() {
            Ok(conf) => {
                target = if let Some(compiler_conf) = conf.get("compiler") {
                    compiler_conf["target"].as_str().unwrap_or(&target).to_string()
                } else {
                    target
                };
                if let Some(target_conf) = conf.get(&target) {
                    match target.as_str() {
                        "makepad" => {
                            let builder = MakepadBuilder::from_config(target_conf).build();
                            Some(Box::new(builder))
                        }
                        _ => {
                         error_and_exit("target compiler has not been implemented yet");
                        }
                    }
                } else {
                    error(&format!(
                        "⛔ {} can not find target {} in gen.toml!",
                        COMPILER_SERVICE, &target
                    ));
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub fn makepad() -> MakepadBuilder {
        let mut target = TARGET.lock().unwrap();
        *target = TargetCompiler::Makepad;
        MakepadBuilder::new(())
    }
    pub fn slint() -> MakepadBuilder {
        let mut target = TARGET.lock().unwrap();
        *target = TargetCompiler::Slint;
        todo!("Slint is not supported yet")
    }
}