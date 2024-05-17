mod core;
mod utils;

pub use core::*;
pub use utils::*;

/// default uncompiled files
/// it will be ignored when compile
/// ### todo!
/// - get uncompiled files from config file (`.genignore`)
const UNCOMPILED: [&str; 5] = [
    "Cargo.toml",
    "main.rs",
    ".gitignore",
    "Cargo.lock",
    "target",
];

/// ## compiler app
/// - path:compile target path (all folders are compiled, files are compiled as single files)
/// ### attention
/// if path is relative path, you should write from project root not the current file
pub fn app(target: Target) -> Compiler {
    // [init log service] --------------------------------------------------------------------------
    let _ =  init_log();
    // [get target watcher path] -------------------------------------------------------------------
    let origin_path = std::env::current_dir().unwrap();
    // [init watcher service] ----------------------------------------------------------------------
    // let _ = init_watcher(origin_path.as_path());

    let is_dir = origin_path.is_dir();

    let target = CompilerTarget::from(target);

    Compiler {
        origin_path,
        is_dir,
        target,
        entry: "app".to_string(),
        root: None,
        exclude: UNCOMPILED.iter().map(|item| item.to_string()).collect(),
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
