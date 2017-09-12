extern crate logian;
use self::logian::schema::parser::*;
use self::logian::schema::data_type::*;

#[test]
pub fn test_parse_empty_schema() {
    let json = r#"[]"#;
    let log_name = "empty_log";
    let actual = parse_log_schema(log_name, json, &vec![]);
    assert_eq!(log_name, actual.name);
    assert!(actual.fields.is_empty());
}

#[test]
pub fn test_parse_one_field_schema() {
    let json = r#"[
        { "name": "field1", "type": "string" }
    ]"#;
    let log_name = "log_name";
    let actual = parse_log_schema(log_name, json, &vec![]);
    assert_eq!(log_name, actual.name);
    assert_eq!("field1", actual.fields[0].name);
    assert_eq!(DataType::String, actual.fields[0].data_type);
}

#[test]
pub fn test_parse_mutiple_field_schema() {
    let json = r#"[
        { "name": "field1", "type": "integer" },
        { "name": "field2", "type": "float" }
    ]"#;
    let log_name = "log_name";
    let actual = parse_log_schema(log_name, json, &vec![]);
    assert_eq!(log_name, actual.name);
    assert_eq!("field1", actual.fields[0].name);
    assert_eq!(DataType::Integer, actual.fields[0].data_type);
    assert_eq!("field2", actual.fields[1].name);
    assert_eq!(DataType::Float, actual.fields[1].data_type);
}

#[test]
pub fn test_parse_log_schema_with_user_defined_type() {
    let type_json = r#"[{ "name": "field1", "type": "string" }]"#;
    let log_json = r#"[{ "name": "field1", "type": "user_defined_type" }]"#;
    let types = parse_user_defined_type("user_defined_type", type_json);
    let log_name = "log_name";
    let actual = parse_log_schema(log_name, log_json, &vec![types]);
    assert_eq!(log_name, actual.name);
    assert_eq!("field1", actual.fields[0].name);
    let field = Field { name: "field1".to_string(), data_type: DataType::String, type_attribute: TypeAttribute::None };
    let expected_type = DataType::Struct("user_defined_type".to_string(), vec![field]);
    assert_eq!(expected_type, actual.fields[0].data_type);
}


#[test]
pub fn test_parse_user_defined_type() {
    let json = r#"[
        { "name": "field1", "type": "integer" },
        { "name": "field2", "type": "float" }
    ]"#;
    let type_name = "type_name";
    let actual = parse_user_defined_type(type_name, json);
    if let DataType::Struct(name, fields) = actual {
        assert_eq!(type_name, name);
        assert_eq!("field1", fields[0].name);
        assert_eq!(DataType::Integer, fields[0].data_type);
        assert_eq!("field2", fields[1].name);
        assert_eq!(DataType::Float, fields[1].data_type);
    } else {
        panic!("Matching failed.")
    }
}

#[test]
pub fn test_parse_default_log_schema() {
    let json = r#"{
        "front": [
            { "name": "front_field1", "type": "string" },
            { "name": "front_field2", "type": "integer" }
        ],
        "back": [
            { "name": "back_field1", "type": "float" },
            { "name": "back_field2", "type": "datetime" }
        ]
    }"#;
    let actual = parse_default_log_schema(json, &vec![]);
    assert_eq!("front_field1", actual.front_fields[0].name);
    assert_eq!(DataType::String, actual.front_fields[0].data_type);
    assert_eq!("front_field2", actual.front_fields[1].name);
    assert_eq!(DataType::Integer, actual.front_fields[1].data_type);
    assert_eq!("back_field1", actual.back_fields[0].name);
    assert_eq!(DataType::Float, actual.back_fields[0].data_type);
    assert_eq!("back_field2", actual.back_fields[1].name);
    assert_eq!(DataType::DateTime, actual.back_fields[1].data_type);
}

#[test]
#[should_panic(expected = "Json root is not an array: log_name.")]
pub fn test_error_to_parse_log_schema_for_not_array() {
    let json = r#"{ "name": "field1", "type": "string" }"#;
    parse_log_schema("log_name", json, &vec![]);
}

#[test]
#[should_panic(expected = "log_name#field1 is not a string type name.")]
pub fn test_error_to_parse_log_schema_for_no_string_type_name() {
    let json = r#"[{ "name": "field1", "type": 1 }]"#;
    parse_log_schema("log_name", json, &vec![]);
}

#[test]
#[should_panic(expected = "Found undefined data type: undefined_type.")]
pub fn test_error_to_parse_log_schema_for_undefined_type() {
    let json = r#"[{ "name": "field1", "type": "undefined_type" }]"#;
    parse_log_schema("log_name", json, &vec![]);
}

#[test]
#[should_panic(expected = "default_schema json root is not an object.")]
pub fn test_error_to_parse_default_log_schema_for_not_object() {
    let json = r#"[]"#;
    parse_default_log_schema(json, &vec![]);
}

#[test]
#[should_panic(expected = "default_schema#field1 is not a string type name.")]
pub fn test_error_to_parse_default_log_schema_for_no_string_type_name() {
    let json = r#"{ "front": [ { "name": "field1", "type": [] } ] }"#;
    parse_default_log_schema(json, &vec![]);
}

#[test]
#[should_panic(expected = "Found undefined data type: undefined_type.")]
pub fn test_error_to_parse_default_log_schema_for_undefined_type() {
    let json = r#"{ "front": [{ "name": "field1", "type": "undefined_type" }] }"#;
    parse_default_log_schema(json, &vec![]);
}

#[test]
#[should_panic(expected = "Json root is not an array: type_name.")]
pub fn test_error_to_parse_user_defined_schema_for_not_array() {
    let json = r#"{ "name": "field1", "type": "string" }"#;
    parse_user_defined_type("type_name", json);
}

#[test]
#[should_panic(expected = "type_name#field1 is not a string type name.")]
pub fn test_error_to_parse_user_defined_type_for_no_string_type_name() {
    let json = r#"[{ "name": "field1", "type": 1 }]"#;
    parse_user_defined_type("type_name", json);
}

#[test]
#[should_panic(expected = "Found undefined data type: undefined_type.")]
pub fn test_error_to_parse_user_defined_type_for_undefined_type() {
    let json = r#"[{ "name": "field1", "type": "undefined_type" }]"#;
    parse_user_defined_type("type_name", json);
}

