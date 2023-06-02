use std::{fs::{self, File}, path::PathBuf, io::Error};

use glob::glob;
use utf8_read::{Reader, Char};

pub fn load_file(path: PathBuf) -> Result<Vec<String>, Error> {
    let file = fs::File::open(path)?;
    let mut reader = utf8_read::Reader::new(&file);

    let mut line: Vec<char> = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    while !reader.eof() {
        let get_char = |reader: &mut Reader<&File>| -> Option<char> {
            let c = reader.into_iter().next_char().ok().expect("I/O error");
            if let Char::Char(c) = c {
                if c != '\r' {
                    Some(c)
                }
                else {
                    reader.into_iter().next_char().ok(); // clear next \n
                    None
                }
            }
            else {
                None
            }
        };
    
        while let Some(c) = get_char(&mut reader) {
            line.push(c);
        }
        let line_s: String = line.clone().into_iter().collect();
        lines.push(line_s);
        line.clear();
    }
    Ok(lines)
}

pub fn find_single(pattern: &str) -> Result<PathBuf, &'static str> {
    for entry in glob(pattern).expect("Invalid glob pattern!") {
        match entry {
            Ok(path) => return Ok(path),
            Err(_) => ()
        }
    }
    return Err("file not found!");
}