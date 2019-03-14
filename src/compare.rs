use std::path::Path;
use std::fs;
use std::io;

use crate::filehash::FileHash;

pub fn hash_directory(path: &Path, level: u32) -> io::Result<FileHash> {
    if !path.is_dir() {
        if let Ok(fh) = FileHash::try_from_file(&path, level) {
            return Ok(fh)
        }
    }
    let mut hash_str = String::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let dir_hash = hash_directory(&entry.path(), level + 1).expect("failed to hash directory");
            //println!("hashed entry at level {}: {}", level, dir_hash.to_str());
            hash_str.push_str(&dir_hash.to_str());
        }
    }
    let hashed_dir = FileHash::try_from_dir(&path, level, &hash_str);
    hashed_dir
}

fn get_dirname(p: &Path) -> io::Result<String> {
    let components: Vec<_> = p
        .components()
        .map(|x| x.as_os_str())
        .collect();
    let dirname = components
        .last()
        .expect("could not parse dir name")
        .to_str()
        .expect("could not parse dir name")
        .to_owned();
    Ok(dirname)
}

fn compare_directories(path1: &Path, path2: &Path) -> io::Result<bool> {
    // return true if directories match
    let h1 = hash_directory(path1, 0)?;
    let h2 = hash_directory(path2, 0)?;
    Ok(h1.to_str() == h2.to_str())
}

fn mirror_dir_name(path: &Path, base_path: &Path) -> Option<String> {
    let dirname = get_dirname(path); // returns Result - replace with .filename()
    match dirname {
        Err(_) => return None,
        Ok(dirname) => {
            let p2 = base_path.clone();
            if let Some(p2) = p2.to_str() {
                let mut p2_extended = p2.to_string();
                p2_extended.push_str("\\");
                p2_extended.push_str(&dirname);
                return Some(p2_extended)
            }
        },
    }
    None
}

pub fn walk_compare(p: &Path, p2: &Path) {
    for entry in fs::read_dir(p).expect("error reading directory") {
        let entry = entry.expect("error parsing directory entry");
        let m = mirror_dir_name(&entry.path(), p2).unwrap();
        let mirror_path = Path::new(&m);
        if mirror_path.exists() {
            let dirs_match = compare_directories(&entry.path(), &mirror_path).unwrap();
            if !dirs_match {
                if entry.path().is_file() {
                    println!("updating {}...", entry.path().to_str().unwrap());
                    fs::copy(entry.path(), mirror_path).expect("error copying file");
                } else {
                    walk_compare(&entry.path(), &mirror_path);
                }
            }
        } else {
            println!("creating {}...", entry.path().to_str().unwrap());
            fs::copy(entry.path(), mirror_path).expect("error creating file");
        }
    }
}