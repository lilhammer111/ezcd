use std::env;
use crate::cst::CONFIG_FILE;

pub fn load_config_file() -> String {
    let home_dir = env::var("HOME").expect("Failed to read the home fir");
    format!("{}/{}", home_dir, CONFIG_FILE)
}