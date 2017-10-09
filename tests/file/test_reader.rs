extern crate logian;
use self::logian::file;

#[test]
pub fn test_read_no_file() {
    assert!(file::read("").is_err());
}

