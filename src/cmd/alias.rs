use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use crate::util::load_config_file;

pub fn alias(dirs: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let alias = dirs[1];
    let path_dirs = &dirs[2..];
    let rela_path = path_dirs.join("/");
    let abs_path = format!(
        "{}/{}",
        env::current_dir()?.display(),
        rela_path
    );
    let abs_path = abs_path.as_str();

    let config_file = load_config_file();
    let config_file = config_file.as_str();
    let mut file = OpenOptions::new().append(true).open(config_file)
        .map_err(|e| format!("Failed to open file of {}: {}", config_file, e))?;
    let formatted_text = format!("{},{},", alias, abs_path);
    let formatted_text = formatted_text.as_str();
    file.write_all(formatted_text.as_bytes())
        .map_err(|e| format!("Failed to write into： {}", e))?;
    file.flush().map_err(|e| format!("Failed to flush file: {}", e))?;

    let resp = format!("Setting the alias of {} to {} succeed.", rela_path, alias);
    Ok(resp)
}
