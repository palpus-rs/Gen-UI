use std::{error::Error, fmt::Display};

#[derive(Debug,Clone)]
pub enum MkError{
    HandleChar(String),
}

impl Error for MkError {
    
}

impl Display for MkError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MkError::HandleChar(s) => f.write_fmt(format_args!("HandleChar to error: {}", s)),
        }
    }
}