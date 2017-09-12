extern crate glob;

use std::io::{Read, BufReader, Error};
use std::fs::File;
use self::glob::glob;

pub fn read_glob(pattern: &str) -> Vec<String> {
    let matches = glob(pattern);
    match matches {
        Ok(paths) =>
            return paths.map(|path| path.unwrap().to_str().unwrap().to_string())
                        .collect::<Vec<String>>(),
        Err(_) => return Vec::new()
    }
}

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
