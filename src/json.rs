extern crate serde_json;

use self::serde_json::{Value};

pub fn parse(data: &str) -> Value {
    let result = serde_json::from_str(data);
    if result.is_err() {
        panic!("Json parse error {}: {}", data, result.unwrap_err());
    }
    result.unwrap()
}

