extern crate serde_json;

use self::serde_json::{Value};

pub fn parse(data: &str) -> Result<Value, String> {
    let result = serde_json::from_str(data);
    match result {
        Ok(value) => Ok(value),
        Err(e) => Err(format!("Json parse error {}: {}", data, e)),
    }
}

