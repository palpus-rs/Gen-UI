use std::path::{Path, PathBuf};

use gen_utils::common::Source;
use gen_utils::compiler::{Builder, CompilerImpl};

use crate::{Cache, Compiler, Ignore, TARGET};

use crate::core::TargetCompiler;

/// # Compiler Builder
/// Compiler Builder is a struct to build a compiler

pub struct CompilerBuilder {
    /// origin path is the project path
    origin_path: PathBuf,
    /// compile target default is makepad
    target: Box<dyn CompilerImpl>,
    /// exclude files or folders
    exclude: Vec<PathBuf>,
    target_compiler: TargetCompiler,
}

impl CompilerBuilder {
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
}

impl Builder for CompilerBuilder {
    type From = Box<dyn CompilerImpl>;

    type To = Compiler;

    fn new(value: Self::From) -> Self {
        let origin_path = std::env::current_dir().unwrap();
        let exclude: Vec<PathBuf> = Ignore::new(origin_path.as_path())
            .expect("ignore file error")
            .into();

        let target_compiler = TARGET.lock().unwrap().clone();

        CompilerBuilder {
            origin_path,
            target: value,
            exclude,
            target_compiler,
        }
    }

    fn build(self) -> Self::To {
        let compiled_path = Source::project_dir_to_compiled(self.origin_path.as_path());
        // [init cache service] -----------------------------------------------------------------------
        let cache = Cache::new(self.origin_path.as_path(), self.target_compiler);
        let mut compiler = Self::To {
            origin_path: self.origin_path,
            compiled_path,
            target: self.target,
            exclude: self.exclude,
            cache,
        };

        let _ = compiler.compile();
        
        compiler
    }
}
