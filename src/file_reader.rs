use std::fs::{self, File};

use utf8_read::{Reader, Char};

pub fn load_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).ok().expect("msg");
    let mut reader = utf8_read::Reader::new(&file);

    let mut line: Vec<char> = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    while !reader.eof() {
        let get_char = |reader: &mut Reader<&File>| -> Option<char> {
            let c = reader.into_iter().next_char().ok().expect("msg");
            if let Char::Char(c) = c {
                if c != '\r' {
                    Some(c)
                }
                else {
                    reader.into_iter().next_char().ok().expect("msg"); // clear next \n
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
    lines
}