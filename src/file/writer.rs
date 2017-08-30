use std::io::{Write, BufWriter, Error};
use std::fs::File;

pub fn write(path: &str, data: &str) -> Result<(), Error> {
    let f = File::create(path)?;
    return write_file(f, data);
}

pub fn write_file(f: File, data: &str) -> Result<(), Error> {
    let mut writer = BufWriter::new(f);
    return writer.write_all(data.as_bytes());
}
