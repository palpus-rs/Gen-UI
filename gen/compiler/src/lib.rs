mod target;
mod utils;
use std::{
    fmt::Debug,
    fs, mem,
    path::{Path, PathBuf},
    process::Command,
};

use gen_converter::model::{Model, Source};
pub use target::CompilerTarget;
use toml_edit::DocumentMut;
pub use utils::*;

pub struct Compiler {
    origin_dir: PathBuf,
    origin_path: PathBuf,
    is_dir: bool,
    target: CompilerTarget,
}

impl Compiler {
    pub fn compile(&self) -> () {
        let _ = self.exist_or_create();
        todo!();
        match &self.target {
            CompilerTarget::Slint => todo!("slint plugin not implemented yet"),
            CompilerTarget::Dioxus => todo!("dioxus plugin not implemented yet"),
            CompilerTarget::Makepad => {
                let model = Model::new(&self.origin_path, &self.origin_dir, self.is_dir).unwrap();
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
        // check the super project is a workspace project or not
        let mut super_path = self.origin_dir.clone();
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
        let compiled_dir = Source::origin_dir_to_compiled(&self.origin_dir);
       
        if !compiled_dir.exists() {
            // use std::process::Command to create a new rust project
            let _ = Command::new("cargo")
                .args(["new", "src-gen"])
                .current_dir(super_path.as_path())
                .status()
                .expect("failed to create src-gen project");
        }

        // read the origin project's Cargo.toml file and move the [dependencies] to the src-gen project except gen's dependencies
        let origin_toml_path = &self.origin_dir.join("Cargo.toml");
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
pub fn app<P>(target: CompilerTarget, path: P) -> Compiler
where
    P: AsRef<Path> + Debug,
{
    let origin_path =
        fs::canonicalize(&path).expect(format!("path not found: {:?}", path).as_str());

    let origin_dir = std::env::current_dir().unwrap();
    let is_dir = origin_path.is_dir();

    Compiler {
        origin_dir,
        origin_path,
        is_dir,
        target,
    }
}
