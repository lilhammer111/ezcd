use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use crate::debug_eprintln;
use crate::error::EzcdError;
use crate::util::load_file;
use crate::cst::CONFIG_PATH;

pub fn find(alias: &str) -> Result<String, Box<dyn Error>> {
    let config_file = load_file(CONFIG_PATH);
    let mut file = File::open(config_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let alias_and_path: Vec<&str> = content.split(",").collect();
    let err = Box::new(io::Error::new(io::ErrorKind::NotFound, "Alias does not exist."));
    alias_and_path.iter().enumerate().find_map(|(idx, &ele)| {
        if ele == alias {
            debug_eprintln!("[EZCD-BIN DEBUG] find output: {}", alias_and_path[idx + 1].to_string());
            Some(alias_and_path[idx + 1].to_string())
        } else {
            None
        }
    }).ok_or_else(|| err as Box<dyn Error>)
}

pub fn splice(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    if dirs.is_empty() {
        return Err(Box::new(EzcdError::EmptyInput));
    }

    let path = dirs.join("/");
    if !Path::new(&path).exists() {
        return Err(Box::new(EzcdError::PathNotExist(path)));
    }
    Ok(path)
}

pub fn suggest(_dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    Err(Box::new(EzcdError::EmptyInput))
}