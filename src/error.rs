use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum EzcdError {
    EmptyInput,
    MismatchedArgs,
    PathNotExist(String),
}

impl Display for EzcdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            EzcdError::EmptyInput => write!(f, "No dir or file provided."),
            EzcdError::MismatchedArgs => write!(f, "Mismatch in number of parameters."),
            EzcdError::PathNotExist(ref path) => write!(f, "Path {} does not exist.", path),
        }
    }
}

impl Error for EzcdError {}
