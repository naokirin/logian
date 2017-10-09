extern crate heck;
extern crate tera;
extern crate serde_json;

use std::collections::HashMap;
use self::tera::{Tera, Error};
use self::heck::{SnakeCase, MixedCase, CamelCase, ShoutySnakeCase};
use self::serde_json::value::{Value, to_value};

pub fn register(tera: &mut Tera) {
    tera.register_filter("snake", snake);
    tera.register_filter("shouty_snake", shouty_snake);
    tera.register_filter("upper_camel", upper_camel);
    tera.register_filter("lower_camel", lower_camel);
}

fn snake(value: Value, _: HashMap<String, Value>) -> Result<Value, Error> {
    let s = try_get_value!("snake", "value", String, value);
    Ok(to_value(&s.to_snake_case()).unwrap())
}
fn shouty_snake(value: Value, _: HashMap<String, Value>) -> Result<Value, Error> {
    let s = try_get_value!("shouty_snake", "value", String, value);
    Ok(to_value(&s.to_shouty_snake_case()).unwrap())
}

fn upper_camel(value: Value, _: HashMap<String, Value>) -> Result<Value, Error> {
    let s = try_get_value!("upper_camel", "value", String, value);
    Ok(to_value(&s.to_camel_case()).unwrap())
}
fn lower_camel(value: Value, _: HashMap<String, Value>) -> Result<Value, Error> {
    let s = try_get_value!("lower_camel", "value", String, value);
    Ok(to_value(&s.to_mixed_case()).unwrap())
}
