extern crate logian;
use self::logian::schema::generator::*;

#[test]
pub fn test_generate_empty_field_log_schema() {
    let st = GeneratedLog {
        template_dir: "template".to_string(),
        fields: vec![],
    };
    let actual = st.generate().unwrap();
    let expected = "[\n    \n]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_one_field_log_schema() {
    let st = GeneratedLog {
        template_dir: "template".to_string(),
        fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            }
        ],
    };
    let actual = st.generate().unwrap();
    let expected = "[\n    { \"name\": \"column1\", \"type\": \"string\" }\n]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_fields_log_schema() {
    let st = GeneratedLog {
        template_dir: "template".to_string(),
        fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            GeneratedField {
                name: "column2".to_string(),
                data_type: "boolean".to_string(),
                nullable: true,
            }

        ],
    };
    let actual = st.generate().unwrap();
    let expected = "[
    { \"name\": \"column1\", \"type\": \"string\" },
    { \"name\": \"column2\", \"type\": \"boolean?\" }
]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_empty_field_type_schema() {
    let st = GeneratedType {
        template_dir: "template".to_string(),
        fields: vec![],
    };
    let actual = st.generate().unwrap();
    let expected = "[\n    \n]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_one_field_type_schema() {
    let st = GeneratedType {
        template_dir: "template".to_string(),
        fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            }
        ],
    };
    let actual = st.generate().unwrap();
    let expected = "[\n    { \"name\": \"column1\", \"type\": \"string\" }\n]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_fields_type_schema() {
    let st = GeneratedType {
        template_dir: "template".to_string(),
        fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            GeneratedField {
                name: "column2".to_string(),
                data_type: "boolean".to_string(),
                nullable: true,
            }

        ],
    };
    let actual = st.generate().unwrap();
    let expected = "[
    { \"name\": \"column1\", \"type\": \"string\" },
    { \"name\": \"column2\", \"type\": \"boolean?\" }
]\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_empty_field_default_log_schema() {
    let st = GeneratedDefaultLog {
        template_dir: "template".to_string(),
        front_fields: vec![],
        back_fields: vec![],
    };
    let actual = st.generate().unwrap();
    let expected = "{
    \"front\": [\n        \n    ],
    \"back\": [\n        \n    ]
}\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_one_field_default_log_schema() {
    let st = GeneratedDefaultLog {
        template_dir: "template".to_string(),
        front_fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            }
        ],
        back_fields: vec![
            GeneratedField {
                name: "column2".to_string(),
                data_type: "boolean".to_string(),
                nullable: true,
            }
        ],
    };
    let actual = st.generate().unwrap();
    let expected = "{
    \"front\": [
        { \"name\": \"column1\", \"type\": \"string\" }
    ],
    \"back\": [
        { \"name\": \"column2\", \"type\": \"boolean?\" }
    ]
}\n";
    assert_eq!(expected, actual);
}

#[test]
pub fn test_generate_fields_default_log_schema() {
    let st = GeneratedDefaultLog {
        template_dir: "template".to_string(),
        front_fields: vec![
            GeneratedField {
                name: "column1".to_string(),
                data_type: "string".to_string(),
                nullable: false,
            },
            GeneratedField {
                name: "column2".to_string(),
                data_type: "boolean".to_string(),
                nullable: true,
            },
        ],
        back_fields: vec![
            GeneratedField {
                name: "column3".to_string(),
                data_type: "datetime".to_string(),
                nullable: false,
            },
            GeneratedField {
                name: "column4".to_string(),
                data_type: "integer".to_string(),
                nullable: false,
            },
        ],
    };
    let actual = st.generate().unwrap();
    let expected = "{
    \"front\": [
        { \"name\": \"column1\", \"type\": \"string\" },
        { \"name\": \"column2\", \"type\": \"boolean?\" }
    ],
    \"back\": [
        { \"name\": \"column3\", \"type\": \"datetime\" },
        { \"name\": \"column4\", \"type\": \"integer\" }
    ]
}\n";
    assert_eq!(expected, actual);
}

