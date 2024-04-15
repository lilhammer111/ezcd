use std::error::Error;
use std::{env, fs};
use std::fs::File;
use crate::util::load_config_file;

pub fn remove(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let alias = dirs[1];
    let config_file = load_config_file();
    let config_file = config_file.as_str();
    let mut content = fs::read_to_string(config_file)?;
    let iter = content.find(alias);

    let mut new_content = String::new();
    let mut times = 0;
    for char in iter {
        if times <= 2 {
            new_content.push(char)
        }
        if char == ',' {
            times+=1
        }
    }

    fs::write(config_file, new_content)?;

    Ok(format!("Removing the alias {} succeed.", alias))
}