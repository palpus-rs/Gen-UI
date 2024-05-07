use std::path::PathBuf;

pub fn current_dir() -> PathBuf{
    std::env::current_dir().unwrap()
}
