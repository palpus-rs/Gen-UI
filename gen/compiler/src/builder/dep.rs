use std::path::{Path, PathBuf};

use crate::{DepType, RemoteDep, RustDependence};

use super::compiler::CompilerBuilder;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RustDependenceBuilder {
    parent: CompilerBuilder,
    name: String,
    version: Option<String>,
    features: Option<Vec<String>>,
    default_features: Option<bool>,
    ty: DepType,
}

impl RustDependenceBuilder {
    pub fn version(mut self, version: &str) -> Self {
        self.version.replace(version.to_string());
        self
    }
    pub fn features(mut self, features: Vec<&str>) -> Self {
        self.features
            .replace(features.into_iter().map(str::to_string).collect());
        self
    }
    pub fn default_features(mut self, default_features: bool) -> Self {
        self.default_features.replace(default_features);
        self
    }
    pub fn crate_io(mut self) -> Self {
        self.ty = DepType::Crate;
        self
    }
    pub fn remote(self) -> RemoteDepBuilder {
        RemoteDepBuilder::from(self)
    }
    pub fn local<P>(self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        LocalDepBuilder::from(self).path(path).build()
    }
    pub fn build(mut self) -> CompilerBuilder {
        let _ = self.parent.push_dep(RustDependence {
            name: self.name,
            version: self.version,
            features: self.features,
            default_features: self.default_features,
            ty: self.ty,
        });

        self.parent
    }
}

impl From<(CompilerBuilder, &str)> for RustDependenceBuilder {
    fn from(value: (CompilerBuilder, &str)) -> Self {
        Self {
            parent: value.0,
            name: String::from(value.1),
            version: None,
            features: None,
            default_features: None,
            ty: DepType::Crate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LocalDepBuilder {
    parent: RustDependenceBuilder,
    path: PathBuf,
}

impl LocalDepBuilder {
    pub fn path<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.path = path.as_ref().to_path_buf();
        self
    }
    pub fn build(mut self) -> RustDependenceBuilder {
        self.parent.ty = DepType::Local(self.path);
        self.parent
    }
}

impl From<RustDependenceBuilder> for LocalDepBuilder {
    fn from(value: RustDependenceBuilder) -> Self {
        Self {
            parent: value,
            path: PathBuf::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RemoteDepBuilder {
    parent: RustDependenceBuilder,
    url: String,
    branch: Option<String>,
    /// HEAD commit of PR (SHA1 hash)
    rev: Option<String>,
    tag: Option<String>,
}

impl RemoteDepBuilder {
    pub fn new(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }
    pub fn branch(mut self, branch: &str) -> Self {
        self.branch.replace(branch.to_string());
        self
    }
    pub fn rev(mut self, rev: &str) -> Self {
        self.rev.replace(rev.to_string());
        self
    }
    pub fn tag(mut self, tag: &str) -> Self {
        self.tag.replace(tag.to_string());
        self
    }
    pub fn build(mut self) -> RustDependenceBuilder {
        self.parent.ty = DepType::Remote(RemoteDep {
            url: self.url,
            branch: self.branch,
            rev: self.rev,
            tag: self.tag,
        });
        self.parent
    }
}

impl From<RustDependenceBuilder> for RemoteDepBuilder {
    fn from(value: RustDependenceBuilder) -> Self {
        Self {
            parent: value,
            url: String::new(),
            branch: None,
            rev: None,
            tag: None,
        }
    }
}
