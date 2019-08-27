use std::env;
use std::error::Error;
// use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

use std::fmt;

#[derive(Debug)]
enum MyError {
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

struct Config {
    filename: String,
    max_level: usize,
}

fn main() -> Result<(), MyError> {
    let args: Vec<String> = env::args().collect();
    // println!("Got args {:?} and len={}", args, args.len());

    let config = configure(args)?;
    match generate_toc(config) {
        Ok(_) => Ok(()),
        Err(e) => Err(MyError::IoError(e)),
    }
}

fn configure(args:Vec<String>) -> Result<Config, MyError> {
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

fn generate_toc(config:Config) -> Result<(), io::Error> {
    let f = File::open(config.filename)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        detect_heading(line.unwrap(), config.max_level);
    }
    Ok(())
}

fn detect_heading(line:String, level:usize) {
    let heading_re = Regex::new(r"^(#+)\s+(.+)$").unwrap();
    let matches: Vec<_> = heading_re.captures_iter(&line).collect();
    if matches.len() == 0 {
        return;
    }

    let capture = &matches[0];
    let indent = (capture[1].len() - 1) as usize;
    if indent >= level {return;}
    let title:String = capture[2].to_string();
    println!("{}* [{}](#{})", " ".repeat(indent * 2), title, header_link(&title));
}

fn header_link(title:&String) -> String {
    // note, I don't make sure the links are unique (yet)
    let safe_re = Regex::new(r"[^A-Za-z0-9 \-]").unwrap();
    let made_safe = safe_re.replace_all(title, "");
    made_safe.to_lowercase().replace(" ", "-")
}
