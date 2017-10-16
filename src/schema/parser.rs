extern crate serde_json;

use self::serde_json::{ Value, Map };
use ::json;
use ::schema::data_type::*;

fn parse_field_user_defined_type(
    data_type_name: &str,
    user_defined_types: &Vec<DataType>
) -> Result<DataType, String> {
    let user_defined_type = user_defined_types.into_iter().find(|x| {
        if let DataType::Struct(ref user_defined_name, _) = **x {
            user_defined_name == data_type_name
        } else {
            false
        }
    });
    if user_defined_type.is_none() {
        return Err(format!("Found undefined data type: {}.", data_type_name));
    }
    Ok(user_defined_type.unwrap().clone())
}

fn parse_data_type(type_name: &str, user_defined_types: &Vec<DataType>) -> Result<DataType, String> {
    match type_name {
        "boolean"         => Ok(DataType::Boolean),
        "integer"         => Ok(DataType::Integer),
        "float"           => Ok(DataType::Float),
        "string"          => Ok(DataType::String),
        "datetime"        => Ok(DataType::DateTime),
        "timestamp"       => Ok(DataType::Timestamp),
        user_defined_type => parse_field_user_defined_type(user_defined_type, &user_defined_types),
    }
}

fn parse_field(
    log_name: &str,
    value: &Map<String, Value>,
    user_defined_types: &Vec<DataType>
) -> Result<Field, String> {
    if value.is_empty() {
        return Err(format!("Found undefined fields: {}.", log_name));
    }
    let n = &value["name"];
    let t = &value["type"];
    if !n.is_string() {
        return Err(format!("{} is not a string field name.", log_name));
    }
    let n = n.as_str().unwrap();

    if !t.is_string() {
        return Err(format!("{}#{} is not a string type name.", log_name, n));
    }
    let t = t.as_str().unwrap();
    let data_type = parse_data_type(t, user_defined_types)?;

    let mut attribute = TypeAttribute::None;
    if value.contains_key("nullable") {
        if !value["nullable"].is_boolean() {
            return Err(format!("{}#{} set nullable option, but not a boolean value.", log_name, n));
        }
        if value["nullable"].as_bool().unwrap() {
            attribute = TypeAttribute::Nullable;
        }
    }
    Ok(Field { name: n.to_string(), data_type: data_type, type_attribute: attribute })
}

fn parse_fields(
    log_name: &str,
    json_value: &Value,
    user_defined_types: &Vec<DataType>
) -> Result<Vec<Field>, String> {
    if !json_value.is_array() {
        return Err(format!("Json root is not an array: {}.", log_name));
    }

    let parsed = json_value.as_array().unwrap();
    parsed.iter().map(move |value| {
        match value {
            &Value::Object(ref m) => Ok(m),
            _ => Err(format!("Found an unknown json type: {}.", log_name)),
        }
    }).map(move |value| {
        match value {
            Ok(m)  => parse_field(log_name, m, user_defined_types),
            Err(e) => Err(e),
        }
    })
    .collect::<Result<Vec<Field>, String>>()
}

pub fn parse_log_schema(name: &str, json: &str, user_defined_types: &Vec<DataType>) -> Result<LogSchema, String> {
    let parsed = json::parse(json)?;
    let fields = parse_fields(name, &parsed, user_defined_types)?;
    Ok(LogSchema { name: name.to_string(), fields: fields })
}

pub fn parse_default_log_schema(json: &str, user_defined_types: &Vec<DataType>) -> Result<DefaultLogSchema, String> {
    let parsed = json::parse(json)?;
    if !parsed.is_object() {
        return Err("default_schema json root is not an object.".to_string());
    }
    let front_fields_json = parsed.as_object().unwrap().get("front");
    let back_fields_json = parsed.as_object().unwrap().get("back");

    let mut front_fields: Vec<Field> = Vec::new();
    let mut back_fields: Vec<Field> = Vec::new();
    if front_fields_json.is_some() {
        front_fields = parse_fields("default_schema", &front_fields_json.unwrap(), user_defined_types)?;
    }
    if back_fields_json.is_some() {
        back_fields = parse_fields("default_schema", &back_fields_json.unwrap(), user_defined_types)?;
    }

    Ok(DefaultLogSchema { front_fields: front_fields, back_fields: back_fields })
}

pub fn parse_user_defined_type(name: &str, json: &str) -> Result<DataType, String> {
    let parsed = json::parse(json)?;
    let fields = parse_fields(name, &parsed, &vec![])?;
    Ok(DataType::Struct(name.to_string(), fields))
}

