extern crate walkdir;
use std::fs::Permissions;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::os::unix::fs::PermissionsExt;

fn is_executable(meta: Permissions) -> bool {
  (meta.mode() & 0b001001001) != 0
}

fn should_search(entry: &DirEntry) -> bool {
  entry.metadata().map(|m| m.is_file() && is_executable(m.permissions())).unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}

fn main() {
    for entry in WalkDir::new(".").into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()).filter(should_search) {
        println!("{}", entry.path().display());
    }
}
