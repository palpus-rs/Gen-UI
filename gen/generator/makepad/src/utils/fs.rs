use std::{fs::{self, File}, path::Path};

pub fn create_file(path: &Path) -> File{
    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).expect("Failed to create directory");
        }
    }
    File::create(path).expect("create main.rs file failed")
}