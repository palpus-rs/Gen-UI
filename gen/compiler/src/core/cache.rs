use crate::{info, Target};
use rmp_serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Cursor,
    path::{Path, PathBuf},
};

/// ## Gen compile cache
/// use msgpack to serialize and deserialize
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cache {
    /// cache file path
    path: PathBuf,
    /// current os
    os: String,
    /// compile target, default => makepad
    target: Target,
    /// cache values, key is file path, value is file hash value
    values: Option<HashMap<PathBuf, String>>,
}

impl Cache {
    pub fn new<P>(origin_path: P, target: Target) -> Self
    where
        P: AsRef<Path>,
    {
        let mut path = origin_path.as_ref().to_path_buf();
        let _ = path.push(".gen_cache");

        // current instance
        let instance = Self {
            path: path.clone(),
            os: std::env::consts::OS.to_string(),
            target,
            values: None,
        };

        // check cache file is exist? if existed, read and deserialize it to new cache instance and compare to current system
        let cache = Cache::read(path.as_path())
            .map(|v| {
                return if v.is_same_instance(&instance) {
                    // same means no need to create a new cache file, back current
                    v
                } else {
                    // back new instacne
                    instance.clone()
                };
            })
            .unwrap_or_else(|_|{
                // create a new cache file and return instance
                instance.create();
                instance
            });

        // now we can get the cache file instance and know wheather we need to crate a new cache file
        todo!()

        // cache.create();
    }
    /// compare two cache instance is same or not (except values field)
    /// if same return true, else return false
    ///
    pub fn is_same_instance(&self, another: &Cache) -> bool {
        let another_path = another.path.to_str().unwrap();
        let self_path = self.path.to_str().unwrap();

        match (
            self_path.eq(another_path),
            self.os.eq(&another.os),
            self.target.eq(&another.target),
        ) {
            (true, true, true) => true,
            _ => false,
        }
    }
    // read cache file by path and deserialize it to cache instance
    pub fn read<P>(path: P) -> Result<Cache, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        return if path.as_ref().exists() {
            let f = File::open(path)?;
            let mut decode = Deserializer::new(f);
            let cache: Cache = Deserialize::deserialize(&mut decode)?;
            Ok(cache)
        } else {
            // cache file not exist
            Err("cache file not exist".into())
        };
    }

    // check current system and init the cache file if sys changed
    pub fn create(&self) -> () {
        let cache_path = self.path.as_path();
        // create a new file or open the existed file
        let mut file = if !cache_path.exists() {
            info("creating a new cache file ...");
            File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(cache_path)
        } else {
            File::open(cache_path)
        }
        .expect("failed to create cache file");

        // write the cache file
    }
}
