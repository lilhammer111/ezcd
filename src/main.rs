mod path_parser;
mod error;

use std::env;
use crate::path_parser::join_path;

fn main() {
    let dirs_or_file: Vec<String> = env::args().skip(1).collect();
    match join_path(dirs_or_file) {
        Ok(path) => print!("{}", path),
        Err(e) => {
            eprintln!("Error joining path: {}", e);
            std::process::exit(1);
        }
    }
}
