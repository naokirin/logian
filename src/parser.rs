use std::path::Path;

pub fn parse_user_defined_types(template_dir: &str) -> Vec<::schema::data_type::DataType> {
    let type_files = ::file::read_glob(&format!("{}/**/*.json", template_dir)[..]);
    type_files.into_iter().map(|path| {
        let json = ::file::read(&path[..]).unwrap();
        let path = Path::new(&path[..]);
        let type_name = path.file_stem().unwrap().to_str().unwrap();
        ::schema::parser::parse_user_defined_type(type_name, &json[..])
    }).collect()
}

pub fn parse_log_schemas(template_dir: &str, user_defined_types: &Vec<::schema::data_type::DataType>) -> Vec<::schema::data_type::LogSchema> {
    let schema_files = ::file::read_glob(&format!("{}/**/*.json", template_dir)[..]);
    schema_files.into_iter().map(|path| {
        let json = ::file::read(&path[..]).unwrap();
        let path = Path::new(&path[..]);
        let log_name = path.file_stem().unwrap().to_str().unwrap();
        ::schema::parser::parse_log_schema(log_name, &json[..], &user_defined_types)
    }).collect()
}

pub fn parse_default_log_schema(template_path: &str, user_defined_types: &Vec<::schema::data_type::DataType>) -> ::schema::data_type::DefaultLogSchema {
    let schema = ::file::read(template_path);
    if schema.is_err() {
        panic!("No such default schema file: {}.", template_path);
    }
    let json = schema.unwrap();
    ::schema::parser::parse_default_log_schema(&json[..], &user_defined_types)
}

