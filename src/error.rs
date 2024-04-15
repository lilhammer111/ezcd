use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum PathError {
    EmptyInput,
    PathDoesNotExist(String),
}

impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PathError::EmptyInput => write!(f, "No dir or file provided"),
            PathError::PathDoesNotExist(ref path) => write!(f, "Path does not exist: {}", path),
        }
    }
}

impl Error for PathError {}
