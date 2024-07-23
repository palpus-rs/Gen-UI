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

// #[derive(Debug)]
// pub enum CompilerTarget {
//     Slint,
//     Dioxus,
//     Makepad(MakepadCompiler),
// }

// impl CompilerTarget {
//     pub fn init<P>(&mut self, entry: &str, path: P, root: Option<&PathBuf>) -> ()
//     where
//         P: AsRef<Path>,
//     {
//         match self {
//             CompilerTarget::Slint => todo!("Slint is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let None = makepad {
//                     let instance = Makepad::new(entry, path, root);
//                     makepad.replace(instance);
//                 }
//             }
//         }
//     }
//     pub fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) -> () {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let Some(makepad) = makepad {
//                     makepad.compile(gen_files);
//                 }
//             }
//         }
//     }
//     pub fn get(&self, key: &Source) -> Option<ModelNode> {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let Some(makepad) = makepad {
//                     makepad.get(key)
//                 } else {
//                     None
//                 }
//             }
//         }
//     }
//     pub fn default_dependencies(&self) -> Vec<RustDependence> {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(_) => {
//                 let dep = RustDependence::new("makepad-widgets");
//                 vec![dep]
//             }
//         }
//     }
//     pub fn set_wasm<W>(&mut self, wasm: Box<W>) -> ()
//     where
//         W: WasmImpl,
//     {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let Some(makepad) = makepad {
//                     makepad.set_wasm(wasm);
//                 }
//             }
//         }
//     }
//     pub fn check_wasm(&self) -> Result<bool, Errors> {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let Some(makepad) = makepad {
//                     makepad.check_wasm()
//                 } else {
//                     panic!("Makepad is not initialized")
//                 }
//             }
//         }
//     }
//     pub fn fresh_wasm<P>(&self, path: P) -> Result<Child, Errors>
//     where
//         P: AsRef<Path>,
//     {
//         match self {
//             CompilerTarget::Slint => todo!("Slint Compiler is not supported yet"),
//             CompilerTarget::Dioxus => todo!("Dioxus Compiler is not supported yet"),
//             CompilerTarget::Makepad(makepad) => {
//                 if let Some(makepad) = makepad {
//                     makepad.wasm.as_ref().unwrap().run(path)
//                 } else {
//                     panic!("Makepad is not initialized")
//                 }
//             }
//         }
//     }
// }

// impl From<Target> for CompilerTarget {
//     fn from(value: Target) -> Self {
//         match value {
//             Target::Slint => CompilerTarget::Slint,
//             Target::Dioxus => CompilerTarget::Dioxus,
//             Target::Makepad => CompilerTarget::Makepad(None),
//         }
//     }
// }

// pub enum TargetCompiler{
//     Makepad()
// }
