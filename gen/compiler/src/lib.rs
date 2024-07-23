//! # Gen Compiler
//! Gen Compiler is a tool to compile gen-ui project to target project.
//! ## Features
//! - [x] support Makepad
//! - [ ] support ArkUI
//! - [x] gen cache
//! - [x] gen ignore
//! - [x] gen logger
//! - [x] gen watcher
//! - [ ] continuous construction (no panic when compiling | panic reload)
mod builder;
mod core;
mod utils;

use builder::CompilerBuilder as UnifiedBuilder;
use lazy_static::lazy_static;
pub use core::*;
use std::sync::Mutex;
pub use utils::*;

pub type MakepadBuilder = makepad_gen_plugin::compiler::builder::CompilerBuilder;
pub use gen_utils::compiler::Builder;
use gen_utils::compiler::CompilerImpl;

lazy_static!{
    static ref TARGET: Mutex<TargetCompiler> = Mutex::new(TargetCompiler::Makepad);
}

/// ## compiler app
/// create an app compiler and specify the target
/// ### Attention
/// you should write from project root path as relative path
/// ### Example
/// ```rust
/// use gen_compiler::{app, Target, Builder};
///
/// fn main() {
///     let compiler = Target::makepad()
///         .entry("app")
///         .root("E:/Rust/try/makepad/Gen-UI/examples/gen_makepad_simple/ui/views/root.gen")
///         .add_dep("makepad-widgets")
///         .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
///         .build()
///         .wasm() // do not use if you don't need wasm
///         .build()
///         .build();
///
///     // set app and specify target
///     let mut app = app(Box::new(compiler)).build();
///
///     let _ = app.run();
/// }
///
/// ```
pub fn app(compiler: Box<dyn CompilerImpl>) -> UnifiedBuilder {
    // [init log service] --------------------------------------------------------------------------
    let _ = init_log();

    UnifiedBuilder::new(compiler)
}

#[cfg(test)]
mod test_compiler {
    use gen_utils::compiler::Builder;
    use std::path::PathBuf;

    #[test]
    fn app_build_test() {
        let compiler = super::Target::makepad()
            .entry("app")
            .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
            .add_dep("makepad-widgets")
            .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
            .build()
            .wasm()
            .no_fresh()
            .port(4568)
            .build()
            .build();

        let _app = super::app(Box::new(compiler));
    }

    #[test]
    fn end_gen() {
        let path = PathBuf::from("src_gen/main.gen");
        assert_eq!(path.to_str().unwrap().ends_with(".gen"), true);
    }
}
