use std::path::Path;
use std::fs;

use crate::compare::walk_compare;

pub mod filehash;
mod compare;


fn main() {
    let mut original = "";
    let mut backup = "";
    let fname = "config.txt";
    let buffer = fs::read_to_string(fname).expect("error reading config");
    for (i, line) in buffer.lines().enumerate() {
        if i == 0 {
            original = line;
        } else if i == 1 {
            backup = line;
        }
    }
    println!("backing up {} to {}", original, backup);
    let original = Path::new(original);
    let backup = Path::new(backup);
    walk_compare(&original, &backup);
}
