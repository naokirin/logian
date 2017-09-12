extern crate serde_json;

use self::serde_json::*;
use ::json::parser::*;
use ::schema::data_type::*;

fn parse_field_user_defined_type(data_type_name: &str, user_defined_types: &Vec<DataType>) -> DataType {
    let user_defined_type = user_defined_types.into_iter().find(|x| {
        if let DataType::Struct(ref user_defined_name, _) = **x {
            user_defined_name == data_type_name
        } else {
            false
        }
    });
    if user_defined_type.is_none() {
        panic!("Found undefined data type: {}.", data_type_name);
    }
    user_defined_type.unwrap().clone()
}

fn parse_data_type(type_name: &str, user_defined_types: &Vec<DataType>) -> DataType {
    match type_name {
        "bool"            => DataType::Boolean,
        "integer"         => DataType::Integer,
        "float"           => DataType::Float,
        "string"          => DataType::String,
        "datetime"        => DataType::DateTime,
        "timestamp"       => DataType::Timestamp,
        user_defined_type => parse_field_user_defined_type(user_defined_type, &user_defined_types),
    }
}

fn parse_field(log_name: &str, value: &Map<String, Value>, user_defined_types: &Vec<DataType>) -> Field {
    if value.is_empty() {
        panic!("Found undefined fields: {}.", log_name);
    }
    let n = &value["name"];
    let t = &value["type"];
    if !n.is_string() {
        panic!("{} is not a string field name.", log_name);
    }
    let n = n.as_str().unwrap();

    if !t.is_string() {
        panic!("{}#{} is not a string type name.", log_name, n);
    }
    let t = t.as_str().unwrap();
    let data_type = parse_data_type(t, user_defined_types);

    // TODO: aquire type_attribute from schema json
    Field { name: n.to_string(), data_type: data_type, type_attribute: TypeAttribute::None }
}

fn parse_fields(log_name: &str, json_value: &Value, user_defined_types: &Vec<DataType>) -> Vec<Field> {
    if !json_value.is_array() {
        panic!("Json root is not an array: {}.", log_name);
    }

    let parsed = json_value.as_array().unwrap();
    let columns = parsed.iter().map(move |value| {
        match value {
            &Value::Object(ref m) => m,
            _ => panic!("Found an unknown json type: {}.", log_name),
        }
    }).map(move |value| parse_field(log_name, value, user_defined_types));

    columns.collect()
}

pub fn parse_log_schema(name: &str, json: &str, user_defined_types: &Vec<DataType>) -> LogSchema {
    let parsed = parse_json(json);
    let fields = parse_fields(name, &parsed, user_defined_types);
    LogSchema { name: name.to_string(), fields: fields }
}

pub fn parse_default_log_schema(json: &str, user_defined_types: &Vec<DataType>) -> DefaultLogSchema {
    let parsed = parse_json(json);
    if !parsed.is_object() {
        panic!("default_schema json root is not an object.");
    }
    let front_fields_json = parsed.as_object().unwrap().get("front");
    let back_fields_json = parsed.as_object().unwrap().get("back");

    let mut front_fields: Vec<Field> = Vec::new();
    let mut back_fields: Vec<Field> = Vec::new();
    if front_fields_json.is_some() {
        front_fields = parse_fields("default_schema", &front_fields_json.unwrap(), user_defined_types);
    }
    if back_fields_json.is_some() {
        back_fields = parse_fields("default_schema", &back_fields_json.unwrap(), user_defined_types);
    }

    DefaultLogSchema { front_fields: front_fields, back_fields: back_fields }
}

pub fn parse_user_defined_type(name: &str, json: &str) -> DataType {
    let parsed = parse_json(json);
    let fields = parse_fields(name, &parsed, &vec![]);
    DataType::Struct(name.to_string(), fields)
}

