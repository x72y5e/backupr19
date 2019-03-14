use std::io;
use std::fs;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


#[derive(Debug)]
pub struct FileHash {
    level: u32,
    hash: u64,
    name: String,
}

impl FileHash {
    pub fn try_from_file(path: &Path, level: u32) -> io::Result<Self> {
        let components: Vec<_> = path
            .components()
            .map(|x| x.as_os_str())
            .collect();
        let fname = components
            .last()
            .expect("could not parse filename")
            .to_str()
            .expect("could not parse filename")
            .to_owned();
        let contents = fs::read(path).expect("could not read file");
        let mut hasher = DefaultHasher::new();
        contents.hash(&mut hasher);
        Ok(FileHash { level, hash: hasher.finish(), name: fname })
    }

    pub fn try_from_dir(path: &Path, level: u32, hashes: &str) -> io::Result<Self> {
        let mut hasher = DefaultHasher::new();
        let components: Vec<_> = path
            .components()
            .map(|x| x.as_os_str())
            .collect();
        let dirname = components
            .last()
            .expect("could not parse dir name")
            .to_str()
            .expect("could not parse dir name")
            .to_owned();
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