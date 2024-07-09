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
    /// check wasm or not
    pub wasm_check: bool,
    /// fresh wasm , automatically recompile wasm
    pub wasm_fresh: bool,
    /// wasm port
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
    /// ## set entry file name
    ///
    /// default name is `app`, you can set your entry file name is you don't use `app`
    ///
    /// the entry file means the project main file
    /// ### Makepad
    /// ```txt
    /// |---- src
    /// |---------- app.rs // entry file
    /// |---------- lib.rs
    /// |---------- main.rs
    /// ```
    /// ### Example
    /// ```rust
    /// let app = app(Target::Makepad).entry("app").build();
    /// ```
    pub fn entry(mut self, entry: &str) -> Self {
        self.entry = entry.to_string();
        self
    }
    /// ## set root path of the project
    ///
    /// ### param
    /// - path: do not use absolute path use relative path
    /// ### Example
    /// ```rust
    /// let app = app(Target::Makepad)
    /// .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
    /// .build();
    /// ```
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
    /// ## set compiler result target
    /// now GenUI just support `Makepad`
    pub fn target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
    /// ## set compiler exclude files or folders
    /// In fact, you should rarely call this method.
    /// The best way is to copy the following default ignore content and write it into the `.gen_ignore` file
    /// ### attention
    ///  **if you use this method, you should use relative path**
    /// ### Default Ignores
    /// ```txt
    /// Cargo.toml
    /// src/main.rs
    /// .gitignore
    /// Cargo.lock
    /// target
    /// .gen_cache
    /// .gen_ignore
    /// target
    /// ```
    /// ### .gen_ignore
    /// In `.gen_ignore`, there are files or directories that GenUI projects need to ignore for monitoring
    /// - Using relative paths relative to the current GenUI project directory
    /// - Accurate to a certain file, please do not ignore it in a way similar to `**/*.xxx`
    /// - Use line breaks for segmentation
    /// - When you don't add, the following content will be ignored by default
    pub fn exclude(mut self, excludes: Vec<PathBuf>) -> Self {
        self.exclude = excludes;
        self
    }
    /// ## add exclude file or folder
    /// add exclude file or folder to the exclude list
    pub fn add_exclude<P>(mut self, exclude: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.exclude.push(exclude.as_ref().to_path_buf());
        self
    }
    /// ## add a rust dependence
    /// add rust dependence to the compile result project
    /// 
    /// see [Rust Dependence](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
    pub fn add_dep(self, name: &str) -> RustDependenceBuilder {
        RustDependenceBuilder::from((self, name))
    }
    /// ## 👎push dependence
    /// recommand use `add_dep` method
    pub fn push_dep(&mut self, dep: RustDependence) -> () {
        self.dependencies.push(dep);
    }
    /// ## set wasm
    /// - set wasm check
    /// - set wasm fresh
    /// - set wasm port
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
