use std::error::Error;
use std::fs::File;
use std::{fs, io};
use std::env::current_dir;
use std::io::Read;
use std::path::Path;
use crate::debug_eprintln;
use crate::error::EzcdError;
use crate::util::load_file;
use crate::cst::CONFIG_PATH;
use crate::log;

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

// 用户输入的命令是 ezcd Codes Rus，然后按Tab键，
// ezcd-bin接收到的参数是ezcd-bin --suggest ezcd Codes Rus，
// main函数中已经skip了ezcd-bin，
// 所以suggest函数中传入的args是--suggest ezcd Codes Rus
pub fn suggest(args: Vec<&str>) -> Result<String, Box<dyn Error>> {
    if args.len() < 3 {
        return Err(Box::new(EzcdError::MismatchedArgs))
    }

    let path_prefix: String;
    if let Ok(path) = current_dir() {
        path_prefix = path.to_string_lossy().to_string()
    } else {
        return Err(
            Box::new(
                io::Error::new(io::ErrorKind::Other, "Failed to get current directory.")
            )
        );
    }

    let parent_path = match args.len() {
        3 => { path_prefix }
        _ => {
            let rela_path = args[2..args.len()-1].join("/");
            let log_info =format!("rela_path: {}",rela_path);
            log::write(
                Box::new(EzcdError::Other(log_info))
            );
            format!(
                "{}/{}",
                path_prefix,
                rela_path
            )
        }
    };

    let log_info =format!("parent_path: {}",parent_path);
    log::write(
        Box::new(EzcdError::Other(log_info))
    );

    let mut dirs = Vec::new();

    let entries = fs::read_dir(parent_path)?;
    for entry in entries {
        let son_entry = entry?.path();
        if son_entry.is_dir() {
            if let Some(dir) = son_entry.file_name() {
                if let Some(dir) = dir.to_str() {
                    let completion = args[args.len()-1];
                    if dir.starts_with(completion) {
                        dirs.push(dir.to_owned());
                    }
                }
            }
        }
    }

    let ret = dirs.join(" ");
    let log_info =format!("ret: {}",ret);
    log::write(
        Box::new(EzcdError::Other(log_info))
    );
    Ok(ret)
}