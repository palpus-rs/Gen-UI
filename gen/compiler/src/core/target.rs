#[allow(unused_imports)]
use std::{
    default,
    path::{Path, PathBuf},
};

use gen_utils::compiler::Builder;

use crate::{MakepadBuilder, TARGET};
use serde::{Deserialize, Serialize};

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
