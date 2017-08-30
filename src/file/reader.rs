use std::io::{Read, BufReader, Error};
use std::fs::File;

pub fn read(path: &str) -> Result<String, Error> {
    let f = File::open(path)?;
    return read_file(f);
}

pub fn read_file(f: File) -> Result<String, Error> {
    let mut reader = BufReader::new(f);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    let result = content.clone();
    return Ok(result);
}
