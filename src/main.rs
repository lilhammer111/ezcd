mod error;
mod cmd;
mod util;
mod cst;
#[macro_use]
mod log;

use std::env;
use std::error::Error;
use crate::cmd::alias::{set, list, update, remove};
use crate::cmd::cd::{find, splice, suggest};
use crate::cmd::help::show_help;


fn main() {
    debug_eprintln!("[EZCD-BIN DEBUG] cmd: {:?}", env::args().collect::<Vec<String>>());
    // if entire_args.len() == 1 {
    //     debug_eprintln!("The Arg of ezcd can't be empty.");
    //     std::process::exit(1)
    // }
    // skip the first arg: ezcd-bin
    let args: Vec<String> = env::args().skip(1).collect();
    // let args= args.iter().map(AsRef::as_ref).collect();
    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    debug_eprintln!("[EZCD-BIN DEBUG] args: {:?}", args);

    let func: fn(Vec<&str>) -> Result<String, Box<dyn Error>>;
    match args[0] {
        "--set" => func = set,
        "--update" => func = update,
        "--remove" => func = remove,
        "--help" => func = show_help,
        "--suggest" => func = suggest,
        "--list" => {
            match list() {
                Ok(resp) => {
                    println!("{}", resp);
                    std::process::exit(0)
                }
                Err(e) => {
                    // debug_eprintln!("{}", _e);
                    log::write(e);
                    std::process::exit(1)
                }
            }
        }
        _ => {
            match find(args[0]) {
                Ok(cd_arg) => {
                    print!("{}", cd_arg);
                    std::process::exit(0);
                }
                Err(_) => {
                    match splice(args) {
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
    match func(args) {
        Ok(output) => println!("{}", output),
        Err(e) => {
            // debug_eprintln!("EZCD Error: {}", _e);
            log::write(e);
            std::process::exit(1);
        }
    }
}
