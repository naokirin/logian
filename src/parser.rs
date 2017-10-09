use std::path::Path;

pub fn parse_user_defined_types(template_dir: &str) -> Result<Vec<::schema::data_type::DataType>, String> {
    let type_files = ::file::read_glob(&format!("{}/**/*.json", template_dir)[..]);
    type_files.into_iter().map(|path| {
        let json = ::file::read(&path[..]).unwrap();
        let path = Path::new(&path[..]);
        let type_name = path.file_stem().unwrap().to_str().unwrap();
        ::schema::parser::parse_user_defined_type(type_name, &json[..])
    }).collect::<Result<Vec<_>, _>>()
}

pub fn parse_log_schemas(
    template_dir: &str,
    user_defined_types: &Vec<::schema::data_type::DataType>
) -> Result<Vec<::schema::data_type::LogSchema>, String> {
    let schema_files = ::file::read_glob(&format!("{}/**/*.json", template_dir)[..]);
    schema_files.into_iter().map(|path| {
        let json = ::file::read(&path[..]).unwrap();
        let path = Path::new(&path[..]);
        let log_name = path.file_stem().unwrap().to_str().unwrap();
        ::schema::parser::parse_log_schema(log_name, &json[..], &user_defined_types)
    }).collect::<Result<Vec<_>, _>>()
}

pub fn parse_default_log_schema(
    template_path: &str,
    user_defined_types: &Vec<::schema::data_type::DataType>
) -> Result<::schema::data_type::DefaultLogSchema, String> {
    let schema = ::file::read(template_path);
    if schema.is_err() {
        return Err(format!("No such default schema file: {}.", template_path));
    }
    let json = schema.unwrap();
    ::schema::parser::parse_default_log_schema(&json[..], &user_defined_types)
}

