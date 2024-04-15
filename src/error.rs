use std::fmt::Display;
use std::error::Error;
#[derive(Debug)]
pub enum PathError {
    EmptyInput,
}

impl Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PathError::EmptyInput => write!(f, "No dir or file provided"),
        }
    }
}

impl Error for PathError {}
