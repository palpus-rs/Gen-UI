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
mod core;
mod utils;

pub use core::*;
pub use utils::*;

/// ## compiler app
/// - path: compile target path (all folders are compiled, files are compiled as single files)
/// ### attention
/// if path is relative path, you should write from project root not the current file
/// ### Example
/// ```rust
/// use gen_compiler::{app, DepType, RustDependence, Target};
///
/// fn main() {
///     // set app and specify target
///     let mut app = app(Target::Makepad);
///    // add makepad widget dependence
///    let mut makepad_widget = RustDependence::new("makepad-widgets");
///    makepad_widget.set_ty(DepType::local(
///        "E:/Rust/try/makepad/makepad/rik/makepad/widgets",
///    ));
///    
///    // compile and run
///    let _ = app
///        .entry("app")
///        .root("E:/Rust/try/makepad/Gen-UI/examples/hello/ui/views/root.gen")
///        .add_dep(makepad_widget)
///        .compile();
///
///    let _ = app.run();
/// }
/// ```
pub fn app(target: Target) -> Compiler {
    // [init log service] --------------------------------------------------------------------------
    let _ = init_log();
    // [get target watcher path] -------------------------------------------------------------------
    let origin_path = std::env::current_dir().unwrap();
    // [get ignore file] ---------------------------------------------------------------------------
    let exclude = Ignore::new(origin_path.as_path()).expect("ignore file error").into();
    // [init cache service] -----------------------------------------------------------------------
    let cache = Cache::new(origin_path.as_path(), target);
    let is_dir = origin_path.is_dir();
    // [set compiler target] ----------------------------------------------------------------------
    let target = CompilerTarget::from(target);
    // [return compiler instance] -----------------------------------------------------------------
    Compiler {
        origin_path,
        is_dir,
        target,
        entry: "app".to_string(),
        root: None,
        exclude,
        cache,
        dependencies: Default::default(),
    }
}

#[cfg(test)]
mod test_compiler {
    use std::path::PathBuf;

    #[test]
    fn end_gen() {
        let path = PathBuf::from("src_gen/main.gen");
        assert_eq!(path.to_str().unwrap().ends_with(".gen"), true);
    }
}
