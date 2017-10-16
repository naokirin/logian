#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

mod file;
mod template;
mod schema;
mod json;
mod parser;
mod option;
mod output;
mod plugin;

fn unwrap_result<T>(result: Result<T, String>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => panic!(e),
    }
}

fn convert_generated_field(field: &option::GeneratedField) -> schema::generator::GeneratedField {
    schema::generator::GeneratedField {
        name: field.field_name.clone(),
        data_type: field.data_type.clone(),
        nullable: field.nullable,
    }
}

fn main() {
    let args = option::parse();
    if args.is_type_generate() {
        let param = unwrap_result(args.as_type_generate());
        let _ = unwrap_result(schema::generator::GeneratedType {
            schema_dir: param.schema_dir,
            name: param.name,
            fields: param.fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
        }.generate());

    }
    else if args.is_log_generate() {
        let param = unwrap_result(args.as_log_generate());
        let _ = unwrap_result(schema::generator::GeneratedLog {
            schema_dir: param.schema_dir,
            name: param.name,
            fields: param.fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
        }.generate());
    }
    else if args.is_default_log_generate() {
        let param = unwrap_result(args.as_default_log_generate());
        let _ = unwrap_result(schema::generator::GeneratedDefaultLog {
            schema_dir: param.schema_dir,
            front_fields: param.front_fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
            back_fields: param.back_fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
        }.generate());
    }
    else if args.is_output() {
        let param = unwrap_result(args.as_output());
        let plugin_setting = unwrap_result(plugin::parse_setting(&param.plugin_dir, &param.plugin_name));
        let logian_output = output::LogianOutput {
            plugin_dir: param.plugin_dir.clone(),
            plugin_name: param.plugin_name.clone(),
            schema_dir: param.schema_dir,
            output_dir: param.output_dir.clone(),
            logs_file_name: "logs".to_string(),
            types_file_name: "types".to_string(),
            file_suffix: plugin_setting.file_suffix,
            compiled: plugin_setting.compiled,
            file_name_case: plugin_setting.file_name_case,
        }.output();
        if logian_output.is_err() {
            panic!(logian_output.unwrap_err());
        }

        let _ = plugin::copy_libs(
            &plugin_setting.libs,
            &param.plugin_dir,
            &param.plugin_name,
            &param.output_dir
            );
    }
}
