mod check;
mod config;
mod version;
mod builder;
mod execute;
mod model_node;

use std::{any::Any, path::PathBuf};

pub use check::*;
pub use version::*;
pub use config::*;
pub use builder::*;
pub use execute::*;
pub use model_node::*;

use crate::common::Source;

/// # Compiler Impl
/// each compiler should implement this trait
pub trait CompilerImpl {
    /// ## execute auxiliaries
    /// execute auxiliaries for the compiler, such as:
    /// - fresh wasm
    fn execute_auxiliaries(&mut self, executor: Executor) -> ();
    /// ## exist or create compiled project
    /// check the compiled project path is exist or not
    /// - if exist, you can do some other things, such as check the config files, environment, etc.
    /// - if not exist, you can create the compiled project path
    /// ### more details
    /// see `generator/makepad/src/compiler/mod.rs` to know what we do in this function
    fn exist_or_create(&self) -> ();
    fn before_compile(&mut self) -> ();
    /// ## compile
    /// compile the project
    /// ### more details
    /// see `generator/makepad/src/compiler/target.rs` to know what we do in this function
    fn compile(&mut self, gen_files: Option<&Vec<&PathBuf>>) -> ();
    /// ## insert node into compiler tree
    fn insert(&mut self, node: Box<dyn Any>) -> ();
    /// ## get node from compiler tree
    fn get(&self, key: &Source) -> Option<Box<dyn ModelNodeImpl>>;
}