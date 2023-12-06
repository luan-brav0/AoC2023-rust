use std::fs::File;
use std::io::{self, BufRead};

pub fn lines_to_vec(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(format!("./inputs/{}", file_path).as_str())?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    return Ok(lines) as io::Result<Vec<String>>;
}
