mod target;
mod utils;
use std::{
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use gen_converter::model::Model;
pub use target::CompilerTarget;
pub use utils::*;

pub struct Compiler {
    compiled_dir: PathBuf,
    compiled_path: PathBuf,
    is_dir: bool,
    target: CompilerTarget,
}

impl Compiler {
    pub fn compile(&self) -> () {
        let _ = self.exist_or_create();

        match &self.target {
            CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
            CompilerTarget::Dioxus => todo!("dioxus plugin not implemented yet"),
            CompilerTarget::Makepad => {
                let model =
                    Model::new(&self.compiled_path, &self.compiled_dir, self.is_dir).unwrap();
                let makepad = makepad_gen_plugin::widget::model::Model::new(model);
            }
        }
    }

    /// ## check if the generate rust project exists, if not create one
    ///
    /// ### details
    /// - check if the project exists which named "src-gen"
    ///     - true: return true
    ///     - false: create a new rust project named "src-gen"
    /// - and need to check whether the super project is a rust workspace project
    ///     - if not, panic and tell the user to create a workspace project
    ///     - if true, check and add the "src-gen" project to the workspace member list
    fn exist_or_create(&self) -> () {
        dbg!(&self.compiled_dir);
    }
}

/// ## 编译
/// - path:编译目标路径(文件夹则全部编译，文件则单文件编译)
/// ### attention
/// if path is relative path, you should write from project root not the current file
pub fn app<P>(target: CompilerTarget, path: P) -> Compiler
where
    P: AsRef<Path> + Debug,
{
    let mut compiled_path =
        fs::canonicalize(&path).expect(format!("path not found: {:?}", path).as_str());

    let compiled_dir = std::env::current_dir().unwrap();
    let is_dir = compiled_path.is_dir();

    Compiler {
        compiled_dir,
        compiled_path,
        is_dir,
        target,
    }
}
