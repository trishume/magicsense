extern crate walkdir;
extern crate aho_corasick;
use std::io::prelude::*;
use std::fs::File;
use std::fs::Permissions;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::os::unix::fs::PermissionsExt;
use std::env;
use std::io;
use aho_corasick::{Automaton, AcAutomaton};

static LIB_EXTENSIONS : &'static [ &'static str ] = &[".o", ".dylib", ".a", ".exe", ".lib"];
const BUFFER_SIZE : usize = 4096;

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

fn find_paths(path: &str, extension: &str) -> io::Result<Vec<usize>> {
    let aut = AcAutomaton::new(vec!["/Users/","/usr/","/System/"]);
    let rdr = try!(File::open(path));

    let mut results = vec![];
    for m in aut.stream_find(rdr) {
        let m = try!(m); // could be an IO error
        // println!("Pattern '{}' matched at: ({}, {})", aut.pattern(m.pati), m.start, m.end);
        results.push(m.start);
    }
    Ok(results)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir: &str = if args.len() > 1 { &args[1] } else { "." };
    for entry in WalkDir::new(base_dir).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()).filter(should_search) {
        println!("{}", entry.path().display());
        println!("{:?}", find_paths(entry.path().to_str().unwrap(),".h").unwrap());
    }
}
