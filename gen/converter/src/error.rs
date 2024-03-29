use std::{error::Error, fmt::Display};

#[derive(Debug,Clone, Copy)]
pub enum Errors{
    MissMatchKeyWord,
}


impl Error for Errors {
    
}

impl Display for Errors{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self{
            Errors::MissMatchKeyWord => "Gen-Converter: MissMatchKeyWord",
        })
    }
}