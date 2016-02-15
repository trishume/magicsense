extern crate walkdir;
use std::fs::Permissions;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::os::unix::fs::PermissionsExt;
use std::env;

static LIB_EXTENSIONS : &'static [ &'static str ] = &[".o", ".dylib", ".a", ".exe", ".lib"];

fn is_executable(meta: Permissions) -> bool {
    (meta.mode() & 0b001001001) != 0
}

fn is_good_path(path: &str) -> bool {
    if path.contains(".dSYM/Contents/Resources/DWARF/") { return true; }
    for ext in LIB_EXTENSIONS {
        if path.ends_with(ext) { return true; }
    }
    return false;
}

fn should_search(entry: &DirEntry) -> bool {
    entry.path().to_str().map(|p| is_good_path(p)).unwrap_or(false) ||
    entry.metadata().map(|m| m.is_file() && is_executable(m.permissions())).unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir: &str = if args.len() > 1 { &args[1] } else { "." };
    for entry in WalkDir::new(base_dir).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()).filter(should_search) {
        println!("{}", entry.path().display());
    }
}
