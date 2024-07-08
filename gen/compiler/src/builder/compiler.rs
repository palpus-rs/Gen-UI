use std::path::{Path, PathBuf};

use gen_converter::model::Source;

use crate::{Cache, Compiler, CompilerTarget, Ignore, RustDependence, Target};

use super::{dep::RustDependenceBuilder, wasm::WasmBuilder};

/// # Compiler Builder
/// Compiler Builder is a struct to build a compiler
#[derive(Debug, Clone)]
pub struct CompilerBuilder {
    /// origin path is the project path
    origin_path: PathBuf,
    /// origin path is a dir or a file
    is_dir: bool,
    /// compile target default is makepad
    target: Target,
    /// entry file name, default is app
    entry: String,
    /// root path of the project
    root: Option<PathBuf>,
    /// exclude files or folders
    exclude: Vec<PathBuf>,
    /// rust dependencies in Cargo.toml
    /// it depends on the target
    /// - makepad: makepad-widgets
    /// > **you can add more other dependencies which you need**
    dependencies: Vec<RustDependence>,
    /// use wasm to run ?
    pub wasm: bool,
    pub wasm_check: bool,
    pub wasm_fresh: bool,
    pub wasm_port: Option<u16>,
}

impl From<Target> for CompilerBuilder {
    fn from(value: Target) -> Self {
        let origin_path = std::env::current_dir().unwrap();
        let is_dir = origin_path.is_dir();
        let exclude: Vec<PathBuf> = Ignore::new(origin_path.as_path())
            .expect("ignore file error")
            .into();
        Self {
            origin_path,
            is_dir,
            target: value,
            entry: "app".to_string(),
            root: None,
            exclude,
            dependencies: Default::default(),
            wasm: false,
            wasm_check: false,
            wasm_fresh: true,
            wasm_port: None,
        }
    }
}

impl CompilerBuilder {
    pub fn entry(mut self, entry: &str) -> Self {
        self.entry = entry.to_string();
        self
    }
    /// set root path of the project
    /// do not use absolute path use relative path
    pub fn root<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        // let path = path
        //     .as_ref()
        //     .to_path_buf()
        //     .canonicalize()
        //     .expect("path error");
        // self.root.replace(path);
        self.root.replace(path.as_ref().to_path_buf());
        self
    }
    pub fn target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
    pub fn is_dir(mut self, is_dir: bool) -> Self {
        self.is_dir = is_dir;
        self
    }
    pub fn exclude(mut self, excludes: Vec<PathBuf>) -> Self {
        self.exclude = excludes;
        self
    }
    pub fn add_exclude<P>(mut self, exclude: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.exclude.push(exclude.as_ref().to_path_buf());
        self
    }
    pub fn add_dep(self, name: &str) -> RustDependenceBuilder {
        RustDependenceBuilder::from((self, name))
    }
    pub fn push_dep(&mut self, dep: RustDependence) -> () {
        self.dependencies.push(dep);
    }
    pub fn wasm(self) -> WasmBuilder {
        self.into()
    }
    /// ## build compiler
    /// build compiler with the builder and run compile function
    pub fn build(self) -> Compiler {
        let origin_path = self.origin_path.clone();
        // [init cache service] -----------------------------------------------------------------------
        let cache = Cache::new(origin_path.as_path(), self.target);
        // [set compiler target] ----------------------------------------------------------------------
        let target = CompilerTarget::from(self.target);
        let compiled_path = Source::project_dir_to_compiled(&self.origin_path);
        let mut compiler = Compiler {
            origin_path: self.origin_path,
            is_dir: self.is_dir,
            target,
            entry: self.entry,
            root: self.root,
            exclude: self.exclude,
            dependencies: self.dependencies,
            wasm: self.wasm,
            cache,
            wasm_process: None,
            compiled_path,
        };

        let _ = compiler.compile();

        if self.wasm {
            match self.target {
                Target::Slint => todo!("not support wasm for slint"),
                Target::Dioxus => todo!("not support wasm for dioxus"),
                Target::Makepad => {
                    compiler.wasm(Box::new(makepad_gen_plugin::wasm::Wasm {
                        check: self.wasm_check,
                        fresh: self.wasm_fresh,
                        port: self.wasm_port,
                    }));

                    // check wasm
                    let _ = compiler.target.check_wasm();
                    let _ = compiler.fresh_wasm();
                }
            }
        }

        compiler
    }
}
