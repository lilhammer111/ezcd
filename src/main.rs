mod error;
mod cmd;
mod util;
mod cst;

use std::env;
use std::error::Error;
use crate::cmd::alias::alias;
use crate::cmd::cd::{find, splice};
use crate::cmd::help::show_help;
use crate::cmd::list::list;
use crate::cmd::remove::remove;

fn main() {
    let func: fn(Vec<&str>) -> Result<String, Box<dyn Error>>;
    let args: Vec<String> = env::args().skip(1).collect();
    let dirs: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    match dirs.len() {
        1 => {
            match dirs[0] {
                "--help" => func = show_help,
                "--list" => func = list,
                _ => {
                    match find(dirs[0]) {
                        Ok(cd_arg) => {
                            print!("{}", cd_arg);
                            std::process::exit(0);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        2 => func = remove,
        _ => {
            match dirs[0] {
                "--alias" => func = alias,
                "--update" => func = update,
                _ => func = splice
            }
        }
    }
    // match alias remove list
    match func(dirs) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            std::process::exit(1);
        }
    }
}
