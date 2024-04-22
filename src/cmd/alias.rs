use std::fs;
use std::env::current_dir;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::error::EzcdError;
use crate::util::load_file;
use crate::cst::CONFIG_PATH;

pub fn set(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    if dirs.len() < 2 {
        return Err(Box::new(EzcdError::MismatchedArgs));
    }

    let alias = dirs[1];
    let cur_dir = current_dir()?;
    let abs_path: String;

    if dirs.len() == 2 {
        abs_path = cur_dir.to_string_lossy().into_owned();
    } else {
        let path_dirs = dirs[2..].to_vec();
        abs_path = format!(
            "{}/{}",
            cur_dir.to_str().ok_or("Path contains invalid UTF-8 characters")?,
            path_dirs.join("/"),
        );
    }

    let config_file = load_file(CONFIG_PATH);
    let config_file = config_file.as_str();
    let mut file = OpenOptions::new().append(true).open(config_file)
        .map_err(|e| format!("Failed to open file of {}: {}", config_file, e))?;
    let formatted_text = format!("{},{},", alias, abs_path);
    let formatted_text = formatted_text.as_str();
    file.write_all(formatted_text.as_bytes())
        .map_err(|e| format!("Failed to write into： {}", e))?;
    file.flush().map_err(|e| format!("Failed to flush file: {}", e))?;

    let resp = format!("Setting the alias {} succeed.", alias);
    Ok(resp)
}


pub fn remove(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    if dirs.len() < 2 {
        return Err(Box::new(EzcdError::MismatchedArgs));
    }

    let alias = dirs[1];
    let config_file = load_file(CONFIG_PATH);
    let config_file = config_file.as_str();
    let content = fs::read_to_string(config_file)?;

    let start_index = content.find(alias).ok_or_else(|| {
        format!("Alias '{}' does not exist.", alias)
    })?;

    let mut alias_and_path = String::new();
    let mut nth_comma = 0;
    for char in content[start_index..].chars() {
        if nth_comma < 2 {
            alias_and_path.push(char)
        } else { break; }

        if char == ',' {
            nth_comma += 1
        }
    }
    let new_content = content.replace(&alias_and_path, "");
    fs::write(config_file, new_content)?;

    Ok(format!("Removing the alias {} succeed.", alias))
}


pub fn update(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let copied_dirs = dirs.clone();
    remove(dirs)?;
    set(copied_dirs)
}

pub fn list() -> Result<String, Box<dyn Error>> {
    // read config file
    let config_file = load_file(CONFIG_PATH);
    let mut file = File::open(config_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut resp = String::from("Alias\tAbs Path\n");
    let mut odd_comma = true;
    for c in content.chars() {
        if c == ',' {
            if odd_comma {
                resp.push('\t');
            } else {
                resp.push('\n');
            }
            odd_comma = !odd_comma;
        } else {
            resp.push(c)
        }
    }

    Ok(resp)
}