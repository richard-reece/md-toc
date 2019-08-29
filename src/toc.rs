use crate::error::MyError;
use regex::Regex;

pub fn find_headings(lines: Vec<String>, max_level:usize) -> Vec<String> {
    let mut preformat_block = false;
    let mut result = Vec::new();
    for line in lines {
        if line.starts_with("```") {
            preformat_block = !preformat_block;
            continue
        }
        if !preformat_block {
            match detect_heading(line, max_level) {
                Ok(s) => result.push(s),
                Err(_) => (),
            }
        }
    }   
    result
}

fn detect_heading(line:String, max_level:usize) -> Result<String, MyError> {
    let heading_re = Regex::new(r"^(#+)\s+(.+)$").unwrap();
    let matches: Vec<_> = heading_re.captures_iter(&line).collect();
    if matches.len() == 0 {
        return Err(MyError::Skip);
    }

    let capture = &matches[0];
    let indent = (capture[1].len() - 1) as usize;
    if indent >= max_level {
        return Err(MyError::Skip);
    }
    let title:String = capture[2].to_string();
    Ok(format!("{}* [{}](#{})", " ".repeat(indent * 2), title, header_link(&title)))
}

fn header_link(title:&String) -> String {
    // note, I don't make sure the links are unique (yet)
    let safe_re = Regex::new(r"[^A-Za-z0-9 \-]").unwrap();
    let made_safe = safe_re.replace_all(title, "");
    made_safe.to_lowercase().replace(" ", "-")
}