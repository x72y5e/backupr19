use std::io;
use std::fs;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use fxhash::FxHasher64;


#[derive(Debug)]
pub struct FileHash {
    level: u32,
    hash: u64,
    name: String,
}

impl FileHash {
    pub fn try_from_file(path: &Path, level: u32) -> io::Result<Self> {
        let fname = path
            .file_name()
            .expect("could not read file name")
            .to_str()
            .expect("could not read file name")
            .to_string();
        let contents = fs::read(path).expect("could not read file");
        //let mut hasher = DefaultHasher::new();
        let mut hasher = FxHasher64::default();
        contents.hash(&mut hasher);
        Ok(FileHash { level, hash: hasher.finish(), name: fname })
    }

    pub fn try_from_dir(path: &Path, level: u32, hashes: &str) -> io::Result<Self> {
        //let mut hasher = DefaultHasher::new();
        let mut hasher = FxHasher64::default();
        let dirname = path
            .file_name()
            .expect("could not read directory name")
            .to_str()
            .expect("could not read directory name")
            .to_string();
        hashes.hash(&mut hasher);
        Ok(FileHash { level, hash: hasher.finish(), name: dirname })
    }

    pub fn to_str(&self) -> String {
        let mut mystring = String::new();
        mystring.push_str(&self.level.to_string());
        mystring.push('-');
        mystring.push_str(&self.hash.to_string());
        mystring.push('-');
        mystring.push_str(&self.name);
        mystring
    }
}