extern crate serde_json;

use self::serde_json::{Value};

pub fn parse_json(data: &str) -> Value {
    return serde_json::from_str(data).unwrap();
}

