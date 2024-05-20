use crate::{info, Target};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Write,
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
                return if v.is_same_except_values(&instance) {
                    // same means no need to create a new cache file, back current
                    v
                } else {
                    // back new instacne
                    let _ = instance.write();
                    instance.clone()
                };
            })
            .unwrap_or_else(|_| {
                // create a new cache file and return instance
                let _ = instance.write();
                instance
            });

        // now we can get the cache file instance
        cache
    }
    /// compare two cache instance is same or not (except values field)
    ///
    /// if same return true, else return false
    pub fn is_same_except_values(&self, another: &Cache) -> bool {
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
    /// compare two cache instance is same or not
    /// all field is same return true
    pub fn is_same(&self, another: &Cache) -> bool {
        if self.is_same_except_values(another) {
            return self.values.eq(&another.values);
        } else {
            return false;
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

    // create cache file and write cache instance to it
    pub fn write(&self) -> () {
        let cache_path = self.path.as_path();
        let mut file = if !cache_path.exists() {
            // create a new file
            File::options()
                .write(true)
                .read(true)
                .create_new(true)
                .open(cache_path)
        } else {
            File::open(cache_path)
        }
        .expect("cache file create or open failed");

        let mut buf = Vec::new();

        let _ = self.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let _ = file.write(&buf).expect("cache file write failed");

        info("cache file write success")
    }
    pub fn insert<P>(&mut self, key: P, value: String) -> ()
    where
        P: AsRef<Path>,
    {
        match &mut self.values {
            Some(values) => {
                values.insert(key.as_ref().to_path_buf(), value);
            }
            None => {
                let mut values = HashMap::new();
                values.insert(key.as_ref().to_path_buf(), value);
                self.values = Some(values);
            }
        }
    }
    pub fn clear(&mut self) -> () {
        self.values = None;
    }
    pub fn get<P>(&self, key: P) -> Option<&String>
    where
        P: AsRef<Path>,
    {
        match &self.values {
            Some(values) => values.get(key.as_ref()),
            None => None,
        }
    }
}
