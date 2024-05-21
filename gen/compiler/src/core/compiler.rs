use std::{
    collections::HashSet,
    fs::{self, File},
    mem,
    path::{Path, PathBuf},
    process::{exit, Command},
};

use gen_converter::model::{Model, Source};
use tokio::runtime::Runtime;
use toml_edit::DocumentMut;
use walkdir::WalkDir;

use crate::{copy_file, info, init_watcher, Cache};

use super::{log::error, CompilerTarget};

pub struct Compiler {
    /// origin path is the project path
    pub origin_path: PathBuf,
    /// origin path is a dir or a file
    pub is_dir: bool,
    /// compile target default is makepad
    pub target: CompilerTarget,
    /// entry file name, default is app
    pub entry: String,
    /// root path of the project
    pub root: Option<PathBuf>,
    /// exclude files or folders
    pub exclude: Vec<String>,
    /// gen_cache
    pub cache: Cache,
}

impl Compiler {
    pub fn run(&self) -> () {
        // let mut super_path = self.origin_path.clone();
        // super_path.pop();
        // super_path.push("src_gen");
        // println!("run app ...");
        // let _ = Command::new("cargo")
        //     .arg("run")
        //     .current_dir(super_path.as_path());
        info("App is running ...");
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = init_watcher(self.origin_path.as_path(), |paths|{
                dbg!(&paths);
                // todo!("compile the file , copy to src_gen and write cache")
            }).await {
                // log error and stop the service
                error(e.to_string().as_str());
                return;
            }
        });
        exit(-1);
    }
    pub fn entry(&mut self, entry: &str) -> &mut Self {
        self.entry = entry.to_string();
        self
    }
    /// set the root path of the project(which need to be excluded from the compile target)
    pub fn root<P>(&mut self, path: P) -> &mut Self
    where
        P: AsRef<Path>,
    {
        self.exclude
            .push(path.as_ref().to_str().unwrap().to_string());
        self.root.replace(path.as_ref().to_path_buf());
        self
    }
    pub fn init_compile_target(&mut self) -> () {
        let _ = self
            .target
            .init(&self.entry, self.origin_path.as_path(), self.root.as_ref());
    }
    /// ## compile the project
    /// ### example
    /// ```rust
    /// use gen_compiler::app;
    ///
    /// fn main(){
    ///    let compiler = app(CompilerTarget::Makepad, "./ui");
    ///    compiler.compile();
    /// }
    /// ```
    /// ### tests
    /// - easy compile: 👌
    pub fn compile(&mut self) -> () {
        let _ = self.exist_or_create();
        let _ = self.init_compile_target();
        let mut visited = HashSet::new();
        // after src_gen project created, get compile target and then use plugin logic to rewrite
        Compiler::loop_compile(self, &mut visited);
        // after all files compiled
        let _ = self.target.compile();
    }
    fn loop_compile(compiler: &mut Compiler, visited: &mut HashSet<PathBuf>) {
        // Convert to absolute path
        // let target_path = target.as_ref().canonicalize().unwrap();
        let target_path = compiler.origin_path.as_path().to_path_buf();
        if !visited.insert(target_path.clone()) {
            return;
        }

        for item in WalkDir::new(target_path.as_path())
            .into_iter()
            .filter_map(|d| d.ok())
        {
            let source_path = item.path();
            // check if the file or folder is in the exclude list, if true, skip it
            if compiler.exclude.iter().any(|uncompiled_item| {
                source_path.ends_with(uncompiled_item)
                    || source_path.to_str().unwrap().eq(uncompiled_item)
            }) {
                continue;
            }

            match (
                source_path.is_file(),
                source_path.to_str().unwrap().ends_with(".gen"),
            ) {
                (false, true) | (false, false) => {
                    // is dir should loop compile again
                    Compiler::loop_compile(compiler, visited);
                }
                (true, true) => {
                    // is gen file, use target compiler to compile then copy to the compiled project
                    compiler
                        .cache
                        .exists_or_insert(&source_path)
                        .unwrap()
                        .modify_then(|| {
                            let model = Model::new(&source_path.to_path_buf(), &target_path, false)
                                .unwrap();
                            match &mut compiler.target {
                                CompilerTarget::Makepad(makepad) => {
                                    makepad.as_mut().unwrap().add(model);
                                }
                                CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
                                CompilerTarget::Dioxus => {
                                    todo!("dioxus plugin not implemented yet")
                                }
                            }
                        });
                }
                (true, false) => {
                    // is file but not gen file, directly copy to the compiled project
                    // get the compiled path
                    let compiled_path = Source::origin_file_without_gen(source_path, &target_path);
                    // check and insert into cache
                    let _ = compiler
                        .cache
                        .exists_or_insert(source_path)
                        .unwrap()
                        .modify_then(|| {
                            let _ = copy_file(source_path, compiled_path);
                        });
                }
            }
        }
    }
    /// ## check if the generate rust project exists, if not create one
    ///
    /// ### details
    /// - check if the project exists which named "src_gen"
    ///     - true: return true
    ///     - false: create a new rust project named "src_gen"
    /// - and need to check whether the super project is a rust workspace project
    ///     - if not, panic and tell the user to create a workspace project
    ///     - if true, check and add the "src_gen" project to the workspace member list
    /// ### test
    /// - no src_gen: 👌
    /// - no src_gen and no workspace: 👌
    fn exist_or_create(&self) -> () {
        // check the super project is a workspace project or not
        let mut super_path = self.origin_path.clone();
        super_path.pop();

        let mut super_toml_path = super_path.clone();
        super_toml_path.push("Cargo.toml");
        if !super_toml_path.exists() {
            panic!("Cargo.toml not found in the super project, you should create a workspace project first");
        } else {
            // read the super project's Cargo.toml file and check the workspace member list
            let mut super_toml = fs::read_to_string(super_toml_path.as_path())
                .expect("failed to read super project's Cargo.toml")
                .parse::<DocumentMut>()
                .expect("Failed to parse Cargo.toml");

            let member_list = super_toml
                .get_mut("workspace")
                .expect("workspace not found in Cargo.toml")
                .get_mut("members")
                .expect("members not found in Cargo.toml")
                .as_array_mut()
                .expect("members is not an array");

            // check member list contains the src_gen project or not
            if member_list
                .iter()
                .find(|item| item.as_str().unwrap() == "src_gen")
                .is_none()
            {
                // add the src_gen project to the workspace member list
                // member_list.push(toml::Value::String("src_gen".to_string()));
                member_list.push("src_gen");
            }
            // write back
            fs::write(super_toml_path.as_path(), super_toml.to_string())
                .expect("failed to write super project's Cargo.toml");
        }

        // check the src_gen project exists or not
        let compiled_dir = Source::origin_dir_to_compiled(&self.origin_path);
        if !compiled_dir.exists() {
            // use std::process::Command to create a new rust project
            let status = Command::new("cargo")
                .args(["new", "src_gen"])
                .current_dir(super_path.as_path())
                .status()
                .expect("failed to create src_gen project");

            if !status.success() {
                panic!("failed to create src_gen project");
            }
        }

        // read the origin project's Cargo.toml file and move the [dependencies] to the src_gen project except gen's dependencies
        let origin_toml_path = &self.origin_path.join("Cargo.toml");
        if !origin_toml_path.exists() {
            panic!("Cargo.toml not found in the origin project");
        }
        let origin_toml_content = fs::read_to_string(origin_toml_path.as_path())
            .expect("failed to read origin project's Cargo.toml");
        let origin_toml = origin_toml_content
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml");
        // get the dependencies table and remove the gen's dependencies
        let mut origin_dependencies = origin_toml["dependencies"]
            .as_table()
            .expect("dependencies not found in Cargo.toml")
            .clone();
        origin_dependencies.retain(|k, _| !k.starts_with("gen"));
        // write the dependencies to the src_gen project's Cargo.toml file
        let compiled_toml_path = &compiled_dir.join("Cargo.toml");
        // find the src_gen project's Cargo.toml file's [dependencies] table and replace the origin project's dependencies
        let compiled_toml_content = fs::read_to_string(compiled_toml_path.as_path())
            .expect("failed to read src_gen project's Cargo.toml");
        let mut compiled_toml = compiled_toml_content
            .parse::<DocumentMut>()
            .expect("Failed to parse Cargo.toml");
        let compiled_dependencies = compiled_toml["dependencies"]
            .as_table_mut()
            .expect("dependencies not found in Cargo.toml");

        let _ = mem::replace(compiled_dependencies, origin_dependencies);

        // compiled_dependencies.extend(origin_dependencies.iter());
        // write back
        fs::write(compiled_toml_path.as_path(), compiled_toml.to_string())
            .expect("failed to write src_gen project's Cargo.toml");

        // command add Makepad widget crate : `cargo add makepad-widgets`
        let _ = Command::new("cargo")
            .args(["add", "makepad-widgets"])
            .current_dir(compiled_dir.as_path())
            .status()
            .expect("failed to add makepad-widgets to src_gen project");

        info("src_gen project is created successfully ...");
    }
}
