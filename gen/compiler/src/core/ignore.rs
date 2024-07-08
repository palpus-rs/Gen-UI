use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use std::error::Error;

/// ## GenIgnore
/// ignore files when compile
///
/// the ignores are got from file: `.gen_ignore`, the ignore file is in the project root path
/// ### Example
/// ```
/// Cargo.toml
/// src/main.rs
/// .gitignore
/// Cargo.lock
/// target
/// .gen_cache
/// .gen_ignore
/// ```
pub struct Ignore(pub Vec<PathBuf>);

impl Ignore {
    /// path: project root path
    pub fn new<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let ignore_path = path.as_ref().join(".gen_ignore");

        return if ignore_path.exists() {
            let mut ignore_file = File::open(ignore_path.as_path())?;
            let mut ignore_content = vec![];
            ignore_file.read_to_end(&mut ignore_content)?;

            let ignore_content = String::from_utf8(ignore_content)?;
            let ignore = ignore_content
                .split("\n")
                .filter(|item| !item.is_empty())
                .map(|item| path.as_ref().join(item.trim()))
                .collect::<Vec<PathBuf>>();
            Ok(Ignore(ignore))
        } else {
            let ignore = Ignore(
                vec![
                    "Cargo.toml",
                    "src/main.rs",
                    ".gitignore",
                    "Cargo.lock",
                    "target",
                    ".gen_cache",
                    ".gen_ignore",
                    "target",
                ]
                .iter()
                .map(|item| path.as_ref().join(item))
                .collect::<Vec<PathBuf>>(),
            );
            Ok(ignore)
        };
    }
}

impl From<Ignore> for Vec<PathBuf> {
    fn from(value: Ignore) -> Self {
        value.0
    }
}
