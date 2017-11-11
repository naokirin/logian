#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tera;

use std::fs::create_dir_all;
use std::env;

mod file;
mod template;
mod schema;
mod json;
mod parser;
mod option;
mod output;
mod plugin;

fn unwrap_result<T, U: std::fmt::Debug>(result: Result<T, U>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => panic!(format!("{:?}", e)),
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
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let template_dir = exe_dir.clone().join("template").to_str().unwrap().to_string();

    if args.is_init() {
        let param = unwrap_result(args.as_init());
        let _ = unwrap_result(create_dir_all(&format!("{}/logs", &param.schema_dir)[..]));
        let _ = unwrap_result(create_dir_all(&format!("{}/types", &param.schema_dir)[..]));
        let _ = unwrap_result(file::write(&format!("{}/schema.config", &param.schema_dir)[..],
            &format!("{{\n    \"log_label\": \"{}\"\n}}\n", &param.log_label)[..]));
    }
    else if args.is_type_generate() {
        let param = unwrap_result(args.as_type_generate());
        let _ = unwrap_result(create_dir_all(&format!("{}/types", param.schema_dir)[..]));
        let s = unwrap_result(schema::generator::GeneratedType {
            template_dir: template_dir,
            fields: param.fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
            comment: param.comment,
        }.generate());

        let _ = unwrap_result(file::write(&format!("{}/types/{}.json", param.schema_dir, param.name)[..], &s[..]));
    }
    else if args.is_log_generate() {
        let param = unwrap_result(args.as_log_generate());
        let _ = unwrap_result(create_dir_all(&format!("{}/logs", param.schema_dir)[..]));
        let s = unwrap_result(schema::generator::GeneratedLog {
            template_dir: template_dir,
            fields: param.fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
            comment: param.comment,
        }.generate());

        let _ = unwrap_result(file::write(&format!("{}/logs/{}.json", param.schema_dir, param.name)[..], &s[..]));
    }
    else if args.is_default_log_generate() {
        let param = unwrap_result(args.as_default_log_generate());
        let _ = unwrap_result(create_dir_all(&format!("{}", param.schema_dir)[..]));
        let s = unwrap_result(schema::generator::GeneratedDefaultLog {
            template_dir: template_dir,
            front_fields: param.front_fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
            back_fields: param.back_fields.into_iter().map(|field| convert_generated_field(&field)).collect(),
        }.generate());

        let _ = unwrap_result(file::write(&format!("{}/default.json", param.schema_dir)[..], &s[..]));
    }
    else if args.is_output() {
        let param = unwrap_result(args.as_output());
        let schema_config = schema::config::parse(&file::read(&format!("{}/schema.config", param.schema_dir)[..]).unwrap()[..]);
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
            config: schema_config.unwrap(),
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
