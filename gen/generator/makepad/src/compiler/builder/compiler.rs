use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use gen_utils::{
    common::{RustDependence, Source},
    compiler::{Builder, FromConfig},
};
use toml_edit::Item;

use crate::compiler::{wasm::Wasm, Compiler};

use super::{dep::RustDependenceBuilder, wasm::WasmBuilder};

/// # Compiler Builder
/// Compiler Builder is a struct to build a compiler
#[derive(Debug, Clone)]
pub struct CompilerBuilder {
    /// origin path is the project path
    origin_path: PathBuf,
    /// entry file name, default is app
    entry: String,
    /// root path of the project
    root: Option<PathBuf>,
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

impl Default for CompilerBuilder {
    fn default() -> Self {
        let origin_path = std::env::current_dir().unwrap();

        Self {
            origin_path,
            entry: Default::default(),
            root: Default::default(),
            dependencies: Default::default(),
            wasm: Default::default(),
            wasm_check: Default::default(),
            wasm_fresh: Default::default(),
            wasm_port: Default::default(),
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
    /// ### Makepad
    /// if you use target: `Makepad`, wasm is supported
    /// - wasm check is false
    /// - wasm fresh is true
    /// - wasm port is 8010
    pub fn wasm(self) -> WasmBuilder {
        self.into()
    }
    // /// ## build compiler
    // /// build compiler with the builder and run compile function
    // pub fn build(self) -> Compiler {
    //     todo!()
    //     // let _ = compiler.compile();

    //     // if self.wasm {
    //     //     match self.target {
    //     //         Target::Slint => todo!("not support wasm for slint"),
    //     //         Target::Dioxus => todo!("not support wasm for dioxus"),
    //     //         Target::Makepad => {
    //     //             compiler.wasm(Box::new(Wasm {
    //     //                 check: self.wasm_check,
    //     //                 fresh: self.wasm_fresh,
    //     //                 port: self.wasm_port,
    //     //             }));

    //     //             // check wasm
    //     //             let _ = compiler.target.check_wasm();
    //     //             let _ = compiler.fresh_wasm();
    //     //         }
    //     //     }
    //     // }

    //     // compiler
    // }
}

impl Builder for CompilerBuilder {
    type From = ();
    type To = Compiler;

    fn new(_value: Self::From) -> Self {
        Self::default()
    }

    fn build(self) -> Self::To {
        // get compiled path from origin_path -----------------------------------------------------
        let compiled_path = Source::project_dir_to_compiled(self.origin_path.as_path());
        // wasm -----------------------------------------------------------------------------------
        let wasm = Wasm {
            check: self.wasm_check,
            fresh: self.wasm_fresh,
            port: self.wasm_port,
        };

        Compiler {
            origin_path: self.origin_path,
            compiled_path,
            entry: self.entry,
            root: self.root,
            dependencies: self.dependencies,
            wasm: if self.wasm { Some(wasm) } else { None },
            wasm_process: None,
            target: None,
        }
    }
}

impl FromConfig for CompilerBuilder {
    type From = Item;

    fn from_config(from: &Self::From) -> Self {
        let origin_path = std::env::current_dir().unwrap();
        let entry = from["entry"].as_str().unwrap_or("app").to_string();
        let root = from["root"].as_str().map(|p| PathBuf::from(p));
        let wasm_item = from.get("wasm");

        let (wasm, wasm_check, wasm_fresh, wasm_port) = match wasm_item {
            Some(items) => {
                let wasm = wasm_item.is_some();

                let wasm_check = items["check"].as_bool().unwrap_or(false);
                let wasm_fresh = items["fresh"].as_bool().unwrap_or(true);
                let wasm_port: Option<u16> = items["port"].as_integer().map(|p| p as u16);
                (wasm, wasm_check, wasm_fresh, wasm_port)
            }
            None => (false, false, true, None),
        };
        let dependencies = from["dependencies"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(k, v)| RustDependence::from_str(&format!("{} = {}", k, v.to_string())).unwrap())
            .collect();

        Self {
            origin_path,
            entry,
            root,
            dependencies,
            wasm,
            wasm_check,
            wasm_fresh,
            wasm_port,
        }
    }
}
