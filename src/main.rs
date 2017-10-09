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

fn main() {
    let args = option::parse();
    if args.is_generate() {
        let _ = unwrap_result(args.as_generate());
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
