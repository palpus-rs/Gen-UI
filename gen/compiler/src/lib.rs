mod target;
mod utils;
use std::{
    collections::HashSet,
    fmt::Debug,
    fs, mem,
    path::{Path, PathBuf},
    process::Command,
};

use gen_converter::model::{Model, Source};
use makepad_gen_plugin::ToToken;
pub use target::CompilerTarget;
use toml_edit::DocumentMut;
pub use utils::*;
use walkdir::WalkDir;

const UNCOMPILED: [&str; 5] = [
    "Cargo.toml",
    "main.rs",
    ".gitignore",
    "Cargo.lock",
    "target",
];

pub struct Compiler {
    /// origin path is the project path
    origin_path: PathBuf,
    /// origin path is a dir or a file
    is_dir: bool,
    target: CompilerTarget,
}

impl Compiler {
    pub fn run(&self) -> () {
        let mut super_path = self.origin_path.clone();
        super_path.pop();

        println!("run app ...");
        let _ = Command::new("cargo")
            .arg("run")
            .current_dir(super_path.as_path());
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
    pub fn compile(&self) -> () {
        let _ = self.exist_or_create();
        let mut visited = HashSet::new();
        Compiler::loop_compile(
            self.origin_path.as_path(),
            |source_path, source_dir| -> String {
                let model = Model::new(&source_path, source_dir, self.is_dir).unwrap();
                match &self.target {
                    CompilerTarget::Makepad => {
                        let makepad = makepad_gen_plugin::widget::model::Model::new(model);
                        makepad.to_token_stream().to_string()
                    }
                    CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
                    CompilerTarget::Dioxus => todo!("dioxus plugin not implemented yet"),
                }
            },
            &mut visited,
        );
    }
    fn loop_compile<P, F>(target: P, compile_fn: F, visited: &mut HashSet<PathBuf>)
    where
        P: AsRef<Path>,
        F: Fn(&PathBuf, &PathBuf) -> String + Copy,
    {
        // Convert to absolute path
        // let target_path = target.as_ref().canonicalize().unwrap();
        let target_path = target.as_ref().to_path_buf();
        if !visited.insert(target_path.clone()) {
            return;
        }

        for item in WalkDir::new(target_path.as_path())
            .into_iter()
            .filter_map(|d| d.ok())
        {
            let source_path = item.path();

            if UNCOMPILED
                .iter()
                .any(|&uncompiled_item| source_path.to_str().unwrap().ends_with(uncompiled_item))
            {
                continue;
            }

            match (source_path.is_file(), source_path.ends_with(".gen")) {
                (false, true) | (false, false) => {
                    // is dir should loop compile again
                    Compiler::loop_compile(source_path, compile_fn, visited);
                }
                (true, true) => {
                    // is gen file, use target compiler to compile then copy to the compiled project
                    // let target = PathBuf::from(target.as_ref());
                    let compiled_path = Source::origin_file_to_compiled(source_path, &target_path);
                    let _ = fs::write(
                        compiled_path,
                        compile_fn(&source_path.to_path_buf(), &target_path),
                    )
                    .expect("failed to write compiled file");
                }
                (true, false) => {
                    // let target = PathBuf::from(target.as_ref());
                    // is file but not gen file, directly copy to the compiled project
                    // get the compiled path
                    let compiled_path = Source::origin_file_without_gen(source_path, &target_path);
                    let _ = copy_file(source_path, compiled_path);
                }
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
    /// ### test
    /// - no src-gen: 👌
    /// - no src-gen and no workspace: 👌
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

            // check member list contains the src-gen project or not
            if member_list
                .iter()
                .find(|item| item.as_str().unwrap() == "src-gen")
                .is_none()
            {
                // add the src-gen project to the workspace member list
                // member_list.push(toml::Value::String("src-gen".to_string()));
                member_list.push("src-gen");
            }
            // write back
            fs::write(super_toml_path.as_path(), super_toml.to_string())
                .expect("failed to write super project's Cargo.toml");
        }

        // check the src-gen project exists or not
        let compiled_dir = Source::origin_dir_to_compiled(&self.origin_path);
        if !compiled_dir.exists() {
            // use std::process::Command to create a new rust project
            let status = Command::new("cargo")
                .args(["new", "src-gen"])
                .current_dir(super_path.as_path())
                .status()
                .expect("failed to create src-gen project");

            if !status.success() {
                panic!("failed to create src-gen project");
            }
        }

        // read the origin project's Cargo.toml file and move the [dependencies] to the src-gen project except gen's dependencies
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
        // write the dependencies to the src-gen project's Cargo.toml file
        let compiled_toml_path = &compiled_dir.join("Cargo.toml");
        // find the src-gen project's Cargo.toml file's [dependencies] table and replace the origin project's dependencies
        let compiled_toml_content = fs::read_to_string(compiled_toml_path.as_path())
            .expect("failed to read src-gen project's Cargo.toml");
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
            .expect("failed to write src-gen project's Cargo.toml");
    }
}

/// ## compiler app
/// - path:compile target path (all folders are compiled, files are compiled as single files)
/// ### attention
/// if path is relative path, you should write from project root not the current file
pub fn app(target: CompilerTarget) -> Compiler {
    let origin_path = std::env::current_dir().unwrap();

    let is_dir = origin_path.is_dir();

    Compiler {
        origin_path,
        is_dir,
        target,
    }
}
