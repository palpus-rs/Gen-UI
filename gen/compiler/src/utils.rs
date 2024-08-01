use std::{
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use gen_utils::{common::msg::COMPILER_SERVICE, error::Errors};
use toml_edit::DocumentMut;

use crate::error_and_exit;

/// copy file from source_path to compiled_path
pub fn copy_file<P, Q>(from: P, to: Q) -> ()
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // Extract the directory part from the compiled_path
    if let Some(parent_dir) = to.as_ref().parent() {
        // Check if the directory exists, if not, create it
        if !parent_dir.exists() {
            // Create the directory and any necessary parent directories
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }

    // Copy the file from source_path to compiled_path
    // fs::copy(from, to).expect("Failed to copy file to compiled project");
    let _ = copy_with_retries(from, to, 5, Duration::from_millis(500)).unwrap();
}

/// copy file from source_path to compiled_path with retries
fn copy_with_retries<P, Q>(
    from: P,
    to: Q,
    max_attempts: usize,
    delay: Duration,
) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut attempts = 0;
    loop {
        match fs::copy(from.as_ref(), to.as_ref()) {
            Ok(_) => return Ok(()),
            Err(_) if attempts < max_attempts => {
                attempts += 1;
                thread::sleep(delay);
            }
            Err(e) => return Err(e),
        }
    }
}

/// if path is absolute path, convert to relative path
/// else return path
pub fn absolute_or_path<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    path.canonicalize().unwrap().to_path_buf()
}

/// compare two path is equal
/// if is_strict is true, p1,p2 should exist
/// else p1,p2 anyone is not exist is also return true
pub fn is_eq_path<P>(p1: P, p2: P, is_strict: bool) -> bool
where
    P: AsRef<Path>,
{
    match (p1.as_ref().exists(), p2.as_ref().exists()) {
        (true, true) => p1.as_ref().canonicalize().unwrap() == p2.as_ref().canonicalize().unwrap(),
        _ => !is_strict,
    }
}

/// eq path exclude when p1 can not be find return true
/// else compare p1 and p2
pub fn is_eq_path_exclude<P>(p1: P, p2: P) -> bool
where
    P: AsRef<Path>,
{
    match (p1.as_ref().exists(), p2.as_ref().exists()) {
        (true, true) => p1.as_ref().canonicalize().unwrap() == p2.as_ref().canonicalize().unwrap(),
        (true, false) | (false, false) => false,
        (false, true) => true,
    }
}

/// get gen.toml content and parse to DocumentMut
pub fn gen_conf_toml() -> Result<DocumentMut, Errors> {
    let conf_path = std::env::current_dir().unwrap().join("gen.toml");

    match gen_utils::common::fs::try_exists(conf_path.as_path()) {
        Ok(exist) => {
            if !exist {
                error_and_exit(&format!(
                    "⛔ {} {}",
                    COMPILER_SERVICE, "can not find gen.toml! "
                ))
            }
            // read gen.toml
            return gen_utils::common::fs::read(conf_path.as_path())
                .unwrap()
                .parse::<DocumentMut>()
                .map_err(|e| Errors::ParseError(format!("⛔ {} {:?}", COMPILER_SERVICE, e)));
        }
        Err(e) => error_and_exit(&format!("⛔ {} {:?}", COMPILER_SERVICE, e)),
    }
}

pub fn gen_conf_toml_no_exit() -> Result<DocumentMut, Errors> {
    let conf_path = std::env::current_dir().unwrap().join("gen.toml");
    match gen_utils::common::fs::try_exists(conf_path.as_path()) {
        Ok(exist) => {
            if !exist {
                return Err(Errors::ParseError(format!(
                    "⛔ {} {}",
                    COMPILER_SERVICE, "can not find gen.toml! "
                )));
            }
            // read gen.toml
            return gen_utils::common::fs::read(conf_path.as_path())
                .unwrap()
                .parse::<DocumentMut>()
                .map_err(|e| Errors::ParseError(format!("⛔ {} {:?}", COMPILER_SERVICE, e)));
        }
        Err(e) => {
            return Err(Errors::ParseError(format!(
                "⛔ {} {:?}",
                COMPILER_SERVICE, e
            )))
        }
    }
}
