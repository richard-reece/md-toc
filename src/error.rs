use std::error::Error;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Usage(String),
    IoError(io::Error),
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::IoError(e) => e.fmt(f),
            MyError::Usage(s) => f.write_str(s),
        }
    }
}
