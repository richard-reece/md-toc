mod config;
mod error;
mod toc;

use std::env;

fn main() -> Result<(), error::MyError> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::from_args(args)?;
    match config.generate_toc() {
        Ok(_) => Ok(()),
        Err(e) => Err(error::MyError::IoError(e)),
    }
}
