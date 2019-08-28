use regex::Regex;

pub fn detect_heading(line:String, level:usize) {
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