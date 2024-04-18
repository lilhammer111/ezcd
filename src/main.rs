mod error;
mod cmd;
mod util;
mod cst;
#[macro_use]
mod log;

use std::env;
use std::error::Error;
use crate::cmd::alias::{set, list, update, remove};
use crate::cmd::cd::{find, splice};
use crate::cmd::help::show_help;


fn main() {
    let entire_args: Vec<String> = env::args().collect();
    debug_eprintln!("[EZCD-BIN DEBUG] entire_args: {:?}", entire_args);
    // if entire_args.len() == 1 {
    //     debug_eprintln!("The Arg of ezcd can't be empty.");
    //     std::process::exit(1)
    // }
    // skip the first arg: ezcd-bin
    let args_without_prefix: Vec<String> = env::args().skip(1).collect();
    let dirs: Vec<&str> = args_without_prefix.iter().map(|x| x.as_str()).collect();
    debug_eprintln!("[EZCD-BIN DEBUG] dirs: {:?}", dirs);

    let func: fn(Vec<&str>) -> Result<String, Box<dyn Error>>;
    match dirs[0] {
        "--set" => func = set,
        "--update" => func = update,
        "--remove" => func = remove,
        "--help" => func = show_help,
        "--list" => {
            match list() {
                Ok(resp) => {
                    println!("{}", resp);
                    std::process::exit(0)
                }
                Err(e) => {
                    debug_eprintln!("{}", e);
                    std::process::exit(1)
                }
            }
        }
        _ => {
            match find(dirs[0]) {
                Ok(cd_arg) => {
                    print!("{}", cd_arg);
                    std::process::exit(0);
                }
                Err(_) => {
                    match splice(dirs) {
                        Ok(spliced_path) => {
                            print!("{}", spliced_path);
                            std::process::exit(0);
                        }
                        Err(e) => {
                            println!("EZCD Error: {}", e);
                            // debug_eprintln!("EZCD Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }

    // match set remove update help
    match func(dirs) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            debug_eprintln!("EZCD Error: {}", e);
            std::process::exit(1);
        }
    }
}
