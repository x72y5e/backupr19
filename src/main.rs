use std::path::Path;
use std::fs;

use crate::compare::{walk_compare, sync_deleted_items};

pub mod filehash;
mod compare;


fn main() {
    let mut original = "";
    let mut backup = "";
    let fname = "config.txt";
    if let Ok(buffer) = fs::read_to_string(fname) {
        for (i, line) in buffer.lines().enumerate() {
            if i == 0 {
                original = line;
            } else if i == 1 {
                backup = line;
            } else {
                println!("\nError: Check config.txt is in form:\n\n\
                original_directory_path\nbackup_directory_path\n\n\
                and that both directories exist.");
                return
            }
        }
        println!("\nbacking up {} to {}\n", original, backup);
        let original = Path::new(original);
        let backup = Path::new(backup);
        sync_deleted_items(&original, &backup);
        walk_compare(&original, &backup);
    } else {
        println!("\nError: Check config.txt is in form:\n\n\
        original_directory_path\nbackup_directory_path\n\n\
        and that both directories exist.");
    }
}
