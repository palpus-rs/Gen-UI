//! # File System Utils
//! recommend to use this module to handle the file system operation instead of `std::fs` to control the error and unify
//! ## Interfaces
//! - exists
//! - try_exists
//! - read
//! - write
//! - append
//! - create
//! - create_new
//! - delete
//! - parse_to`<T> T: FromStr`
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    str::FromStr,
};

use crate::error::{Errors, FsError};

/// ## Check the file is exists ?
/// return `true` if the file is exists or `false` if not exists
/// if path is empty return false
/// ### Also
/// If you want to get the Error reason, use `try_exists` which will return `Result<bool, Errors>`
pub fn exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().exists()
}
/// ## Check the file is exists ?
/// - if the file is exists, return `Ok(true)`
/// - if the file is not exists, return `Ok(false)` (empty)
/// - if the file can not be sure exists or not, return `Err` (kind like you have no permission to access the file or the parent directory)
pub fn try_exists<P>(path: P) -> Result<bool, Errors>
where
    P: AsRef<Path>,
{
    path.as_ref()
        .try_exists()
        .map_err(|e| Errors::FsError(FsError::UnExpected(e.to_string())))
}
/// ## Read the file
/// - if the file is exists, return the content of the file as `String`
/// - if the file is not exists, return `Err` (kind like the file can not be found or no permission)
pub fn read<P>(path: P) -> Result<String, Errors>
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(path.as_ref()).map_err(|e| {
        Errors::FsError(FsError::Read {
            path: path.as_ref().to_path_buf(),
            reason: e.to_string(),
        })
    })
}
/// ## Write the file
/// - if the file is exists, write the content to the file(which will overwrite the origin content)
/// - if the file is not exists, create the file and write the content to the file
/// - if the write process is success, return `Ok(())`
/// - if the write process is fail, return `Err` (kind like the file can not be write or no permission)
/// ### Also
/// If you want to append the content to the file, use `append` method
pub fn write<P>(path: P, content: &str) -> Result<(), Errors>
where
    P: AsRef<Path>,
{
    std::fs::write(path.as_ref(), content).map_err(|e| {
        Errors::FsError(FsError::Write {
            path: path.as_ref().to_path_buf(),
            reason: e.to_string(),
        })
    })
}
/// ## Append the content to the file
/// - if the file is exists, append the content to the file
/// - if the file is not exists, create the file and write the content to the file
pub fn append<P>(path: P, content: &str) -> Result<(), Errors>
where
    P: AsRef<Path>,
{
    std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path.as_ref())
        .and_then(|mut file| file.write_all(content.as_bytes()))
        .map_err(|e| {
            Errors::FsError(FsError::Write {
                path: path.as_ref().to_path_buf(),
                reason: e.to_string(),
            })
        })
}
/// ## Create the file
/// - if is exists, return `Err` (kind like the file is exists, back FsError::UnExpected)
/// - if is not exists, create the file and return `Ok(())`
/// - if the permission is not enough, return `Err` (kind like you have no permission to create the file)
/// ### Also
/// if you want to create a new file and do not care about the exists one, use `create_new` method
pub fn create<P>(path: P) -> Result<(), Errors>
where
    P: AsRef<Path>,
{
    std::fs::File::create(path.as_ref())
        .map(|_| ())
        .map_err(|e| Errors::FsError(FsError::UnExpected(e.to_string())))
}
/// ## Remove the file
/// - if the file is exists, remove the file and return `Ok(())`
/// - if the file is not exists, return `Err` (kind like the file can not be found, back FsError::FileNotFound)
/// - if the permission is not enough, return `Err` (kind like you have no permission to remove the file, back FsError::UnExpected)
pub fn delete<P>(path: P) -> Result<(), Errors>
where
    P: AsRef<Path>,
{
    match try_exists(path.as_ref()) {
        Ok(is_exist) => {
            return if is_exist {
                std::fs::remove_file(path.as_ref())
                    .map(|_| ())
                    .map_err(|e| Errors::FsError(FsError::UnExpected(e.to_string())))
            } else {
                Err(Errors::FsError(FsError::FileNotFound(
                    path.as_ref().to_path_buf(),
                )))
            }
        }
        Err(e) => Err(e),
    }
}

/// ## Create the new file
/// - if is exists, remove the exists one and create a new file, return `Ok(())`
/// - if is not exists, create the file and return `Ok(())`
/// - if the permission is not enough, return `Err` (kind like you have no permission to create the file)
/// ### Also
/// if you want to create a file , but if exists one, return `Err`, use `create` method
pub fn create_new<P>(path: P) -> Result<(), Errors>
where
    P: AsRef<Path>,
{
    match delete(path.as_ref()) {
        Ok(_) => create(path),
        Err(e) => Err(e),
    }
}
/// ## Parse the file to `T`
/// - if the file is exists,read and then parse the content to `T`
/// - if the file is not exists, return `Err` (kind like the file can not be found or no permission)
pub fn parse_to<T, P>(path: P) -> Result<T, Errors>
where
    T: FromStr,
    P: AsRef<Path>,
{
    read(path).and_then(|content| {
        content.parse::<T>().map_err(|_| {
            Errors::ParseError(format!("Parse to {} fail", std::any::type_name::<T>()))
        })
    })
}

/// ## Create the file
/// use create_dir_all to create the parent directory if not exists then create the file
/// ### Errors
/// Errors
/// This function will return an error in the following situations, but is not
/// limited to just these cases:
///
/// * If any directory in the path specified by `path`
/// does not already exist and it could not be created otherwise. The specific
/// error conditions for when a directory is being created (after it is
/// determined to not exist) are outlined by [`fs::create_dir`].
///
/// Notable exception is made for situations where any of the directories
/// specified in the `path` could not be created as it was being created concurrently.
/// Such cases are considered to be successful. That is, calling `create_dir_all`
/// concurrently from multiple threads or processes is guaranteed not to fail
/// due to a race condition with itself.
pub fn create_file(path: &Path) -> Result<File, Errors> {
    if let Some(parent_dir) = path.parent() {
        if try_exists(path)? {
            match create_dir_all(parent_dir) {
                Ok(_) => {}
                Err(e) => {
                    return Err(Errors::FsError(FsError::Create {
                        path: parent_dir.to_path_buf(),
                        reason: e.to_string(),
                    }))
                }
            };
        }
    } else {
        return Err(Errors::FsError(FsError::UnExpected(
            "Path has no parent directory".to_string(),
        )));
    }

    File::create(path).map_err(|e| {
        Errors::FsError(FsError::Create {
            path: path.to_path_buf(),
            reason: e.to_string(),
        })
    })
}


#[cfg(test)]
mod test_fs {
    use std::path::PathBuf;

    use super::*;
    #[test]
    fn test_exists() {
        let res = exists(PathBuf::new());
        assert!(!res);
    }
    #[test]
    fn test_try_exists() {
        let res = try_exists(PathBuf::new());
        assert!(res.is_err());
    }
}
