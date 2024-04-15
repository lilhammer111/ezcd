use std::env;
use crate::error::PathError;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};


const EZCD_CONFIG_FILE_PATH: &str = "~/.config/ezcd/aliases.list";

pub fn splice(dirs: &[String]) -> Result<String, Box<dyn Error>> {
    if dirs.is_empty() {
        return Err(Box::new(PathError::EmptyInput));
    }

    let path = dirs.join("/");
    // if !Path::new(&path).exists() {
    //     return Err(Box::new(PathError::PathDoesNotExist(path)));
    // }
    Ok(path)
}

pub fn alias(dirs: &[String]) -> Result<String, Box<dyn Error>> {
    let alias = &dirs[1];
    let path_dirs = &dirs[1..];
    let abs_path = format!("{},{},", env::current_dir()?.display(), splice(path_dirs)?);

    let mut file = OpenOptions::new().append(true).open(EZCD_CONFIG_FILE_PATH)?;
    file.write_all(format!("{},{},", alias, abs_path))

}

pub fn remove(dirs: &[String]) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn list(dirs: &[String]) -> Result<String, Box<dyn Error>> {
    todo!()
}

pub fn find(alias: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(EZCD_CONFIG_FILE_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let alias_and_path:Vec<&str> = content.split(",").collect();
    let err = Box::new(io::Error::new(io::ErrorKind::NotFound,"Alias does not exist."));
    alias_and_path.iter().enumerate().find_map(|(idx, &ele)|{
        if ele == alias {
            Some(alias_and_path[idx+1].to_string())
        } else {
            None
        }
    }).ok_or_else(||err as Box<dyn Error>)
}