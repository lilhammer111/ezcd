use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum EzcdError {
    EmptyInput,
    MismatchedArgs,
    PathNotExist(String),
    #[allow(dead_code)]
    Other(String),
}

impl Display for EzcdError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            EzcdError::EmptyInput => write!(f, "No dir or file provided."),
            EzcdError::MismatchedArgs => write!(f, "Mismatch in number of parameters."),
            EzcdError::PathNotExist(ref path) => write!(f, "Path {} does not exist.", path),
            EzcdError::Other(ref msg) => write!(f, "{}", msg)
        }
    }
}

impl Error for EzcdError {}
