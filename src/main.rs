extern crate walkdir;
extern crate regex;
#[macro_use] extern crate lazy_static;
use std::io::prelude::*;
use std::fs::File;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use std::env;
use std::io;
use std::io::BufReader;
use regex::Regex;

fn has_extension(entry: &DirEntry, ext: &str) -> bool {
    entry.path().to_str().map(|p| p.ends_with(ext)).unwrap_or(false) && entry.metadata().map(|m| m.is_file()).unwrap_or(false)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with(".") && s != ".")
         .unwrap_or(false)
}

fn index_file(path: &str) -> io::Result<()> {
    lazy_static! {
        static ref FN_RE: Regex = Regex::new(r"(?x)^
            [^(]*
            \s([a-zA-Z_?!]+) # Function name
            \(").unwrap();
    }
    let f = try!(File::open(path));
    let f = BufReader::new(f);

    let mut last_fun : String = String::new();
    let mut last_fun_indent : usize = 0;
    let mut in_fun = false;
    let mut last_line_was_fun = false;
    for line in f.lines() {
        let line = try!(line);
        let indent : usize = line.bytes().take_while(|b| b == &(' ' as u8) || b == &('\t' as u8)).count();
        if indent == line.len() { continue; } // ignore blank lines
        // println!("{} last_fun: {} at {} in_fun: {} last_line_was_fun: {}", indent, last_fun, last_fun_indent, in_fun, last_line_was_fun);

        if in_fun {
            if indent > last_fun_indent {
                if last_line_was_fun {
                    println!("{}", last_fun);
                    last_line_was_fun = false;
                }
            } else {
                in_fun = false;
            }
        } else if let Some(caps) = FN_RE.captures(&line) {
            last_fun = String::from(caps.at(1).unwrap());
            last_fun_indent = indent;
            in_fun = true;
            last_line_was_fun = true;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_dir: &str = if args.len() > 1 { &args[1] } else { "." };
    let ext : &str = if args.len() > 2 { &args[2] } else { ".rs" };
    for entry in WalkDir::new(base_dir).into_iter().filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()).filter(|e| has_extension(e, ext)) {
        println!("{}", entry.path().display());
        index_file(entry.path().to_str().unwrap()).unwrap();
    }
}
