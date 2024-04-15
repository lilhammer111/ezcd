use std::path::Path;
use crate::error::PathError;

pub fn join_path(dirs_or_file: Vec<String>) -> Result<String, PathError> {
    if dirs_or_file.is_empty() {
        return Err(PathError::EmptyInput);
    }

    let path = dirs_or_file.join("/");
    if !Path::new(&path).exists() {
        return Err(PathError::PathDoesNotExist(path))
    }
    Ok(path)
}
