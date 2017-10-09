extern crate tera;

use self::tera::{Context, Tera};
use ::schema::data_type::*;
use std::fmt::Debug;

fn unwrap_result<T: Debug, E: Debug>(result: Result<T, E>) -> T {
    if result.is_ok() {
        return result.unwrap();
    }
    panic!("{:?}", result.unwrap_err());
}

pub fn find_templates(template_dir: &str, plugin_name: &str) -> Tera {
    let templates = format!("{}/{}/templates/**/*.tera", template_dir, plugin_name);
    let result = Tera::new(&templates[..]);
    unwrap_result(result)
}

pub fn render_logs(tera: &Tera, template_name: &str, logs: &Vec<LogSchema>, default_log: &DefaultLogSchema, types: &Vec<DataType>) -> String {
    let mut context = Context::new();
    context.add("default_log", &default_log);
    context.add("logs", &logs);
    context.add("types", &types);
    let result = tera.render(template_name, &context);
    unwrap_result(result)
}

pub fn render_log(tera: &Tera, template_name: &str, log: &LogSchema, default_log: &DefaultLogSchema, types: &Vec<DataType>) -> String {
    let mut context = Context::new();
    context.add("default_log", &default_log);
    context.add("log", &log);
    context.add("types", &types);
    let result = tera.render(template_name, &context);
    unwrap_result(result)
}

pub fn render_types(tera: &Tera, template_name: &str, types: &Vec<DataType>) -> String {
    let mut context = Context::new();
    context.add("types", &types);
    let result = tera.render(template_name, &context);
    unwrap_result(result)
}

pub fn render_type(tera: &Tera, template_name: &str, data_type: &DataType) -> String {
    let mut context = Context::new();
    context.add("type", &data_type);
    let result = tera.render(template_name, &context);
    unwrap_result(result)
}
