//! # Gen Compiler
//! Gen Compiler is a tool to compile gen-ui project to target project.
//! ## Features
//! - [x] support Makepad
//! - [ ] support ArkTS
//! - [x] gen cache
//! - [x] gen ignore
//! - [x] gen logger
//! - [x] gen watcher
//! - [ ] continuous construction (no panic when compiling | panic reload)
mod builder;
mod core;
mod utils;

use builder::compiler::CompilerBuilder;
pub use core::*;
pub use utils::*;

/// ## compiler app
/// create an app compiler and specify the target
/// ### Attention
/// you should write from project root path as relative path
/// ### Example
/// ```rust
/// use gen_compiler::{app, Target};
///
/// fn main() {
///     // set app and specify target
///     let mut app = app(Target::Makepad)
///         .entry("app")
///         .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
///         .add_dep("makepad-widgets") // add makepad-widgets dependency
///         .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
///         .build()
///         .build();
///
///     let _ = app.run(); // run app
/// }
///
/// ```
pub fn app(target: Target) -> CompilerBuilder {
    // [init log service] --------------------------------------------------------------------------
    let _ = init_log();
    target.into()
}

#[cfg(test)]
mod test_compiler {
    use std::path::PathBuf;

    #[test]
    fn app_build_test() {
        let app = super::app(super::Target::Makepad)
            .entry("app")
            .root("E:/Rust/try/makepad/Gen-UI/examples/gosim_example/ui/views/root.gen")
            .add_dep("makepad-widgets")
            .local("E:/Rust/try/makepad/makepad/rik/makepad/widgets")
            .build()
            .wasm()
            .no_fresh()
            .port(4568)
            .build();

        dbg!(app);
    }

    #[test]
    fn end_gen() {
        let path = PathBuf::from("src_gen/main.gen");
        assert_eq!(path.to_str().unwrap().ends_with(".gen"), true);
    }
}
