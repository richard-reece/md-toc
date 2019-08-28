use crate::error::MyError;
use crate::toc;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Config {
    filename: String,
    max_level: usize,
}

impl Config {
    pub fn from_args(args:Vec<String>) -> Result<Config, MyError> {
        if args.len() < 2 {
            return Err(MyError::Usage(format!("usage: {} <filename> [max header level]", args[0])));
        }

        let mut config = Config {
            filename: args[1].clone(),
            max_level: 2,
        };

        if args.len() > 2 {
            let r = args[2].parse();
            if r.is_ok() {
                config.max_level = r.unwrap();
            } else {
                return Err(MyError::Usage(String::from("heading level must be a number")));
            }
        }

        Ok(config)
    }

    pub fn generate_toc(&self) -> Result<(), io::Error> {
        let f = File::open(&self.filename)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            toc::detect_heading(line.unwrap(), self.max_level);
        }
        Ok(())
    }
}