mod path;
mod error;
mod base;

use std::env;
use crate::path::{splice, alias, remove, list, find};
use crate::base::show_help;
use std::error::Error;

fn dispatch_cmd(sub_cmd: &String) -> fn(Vec<String>) -> Result<String, Box<dyn Error>> {
    match sub_cmd {
        String::from("alias") => alias,
        String::from("remove") => remove,
        String::from("list") => list,
        _ => splice,
    }
}

fn main() {
    let func: fn(Vec<String>) -> Result<String, Box<dyn Error>>;
    let dirs: Vec<String> = env::args().skip(1).collect();
    match dirs.len() {
        1 => {
            if dirs[0] == "--help" {
                func = show_help;
            } else {
                match find(dirs[0].as_str()) {
                    Ok(output) => {
                        print!("{}", output);
                        std::process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error joining path: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        }
        _ => {
            func = dispatch_cmd(&dirs[1]);
        }
    }

    match func(dirs) {
        Ok(output) => print!("{}", output),
        Err(e) => {
            eprintln!("Error joining path: {}", e);
            std::process::exit(1);
        }
    }
}
