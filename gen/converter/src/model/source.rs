use std::{ffi::OsString, path::PathBuf};

use gen_utils::common::snake_to_camel;

#[derive(Debug, Default, Clone)]
pub struct Source {
    /// source file dir
    pub origin_dir: PathBuf,
    /// source file path
    pub origin_file: PathBuf,
    /// compiled file path
    pub compiled_dir: PathBuf,
    pub compiled_file: PathBuf,
}

impl Source {
    pub fn source_name(&self) -> String {
        let name = self
            .origin_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".gen", "");
        snake_to_camel(&name).unwrap()
    }
    pub fn as_os_str(&self) -> &std::ffi::OsStr {
        self.compiled_file.as_os_str()
    }
    pub fn origin_dir_to_compiled(origin_dir: &PathBuf) -> PathBuf {
        let mut tmp = origin_dir.clone();
        tmp.pop();
        tmp.push("src-gen");
        tmp
    }
}

/// one is for source file path another is for source dir
/// (source file path, source dir path)
impl From<(&PathBuf, &PathBuf)> for Source {
    fn from(value: (&PathBuf, &PathBuf)) -> Self {
        let mut tmp = value.1.clone();
        tmp.pop();

        let strip_path = value.0.strip_prefix(&tmp.as_path()).unwrap();

        let mut target: Vec<OsString> = strip_path.iter().map(OsString::from).collect();

        // 检查是否有足够可以修改
        if !target.is_empty() {
            // 替换第一个
            target[0] = "src-gen".into();
            if let Some(last) = target.last_mut() {
                
                *last = last.to_str().unwrap().replace(".gen", ".rs").into();
            }
        }

        // 使用base和修改后的组件重新构建完整的路径
        let compiled_file = tmp.clone().join(PathBuf::from_iter(target));
        let mut compiled_dir = tmp;
        compiled_dir.push("src-gen");
        Source {
            origin_dir: value.1.clone(),
            origin_file: value.0.clone(),
            compiled_dir,
            compiled_file,
        }
    }
}
