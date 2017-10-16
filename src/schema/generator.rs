extern crate tera;

use std::env;
use std::fmt::Debug;
use std::fs::create_dir_all;
use self::tera::{Tera, Context};
use ::template::renderer::find_templates;
use ::template::filter;
use ::file;

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct GeneratedField {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

pub struct GeneratedLog {
    pub schema_dir: String,
    pub name: String,
    pub fields: Vec<GeneratedField>,
}

pub struct GeneratedDefaultLog {
    pub schema_dir: String,
    pub front_fields: Vec<GeneratedField>,
    pub back_fields: Vec<GeneratedField>,
}

pub struct GeneratedType {
    pub schema_dir: String,
    pub name: String,
    pub fields: Vec<GeneratedField>,
}

fn unwrap_result<T: Debug, E: Debug>(result: Result<T, E>) -> T {
    if result.is_ok() {
        return result.unwrap();
    }
    panic!("{:?}", result.unwrap_err());
}

fn create_tera() -> Tera {
    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let template_dir = exe_dir.clone().join("template").to_str().unwrap().to_string();
    let mut tera = find_templates(&template_dir[..]);
    filter::register(&mut tera);
    return tera;
}

impl GeneratedLog {
    pub fn generate(&self) -> Result<(), String> {
        let tera = create_tera();
        let mut context = Context::new();
        context.add("fields", &self.fields);
        let s = unwrap_result(tera.render("log_schema.tera", &context));
        let _ = unwrap_result(create_dir_all(&format!("{}/logs", self.schema_dir)[..]));
        let _ = unwrap_result(file::write(&format!("{}/logs/{}.json", self.schema_dir, self.name)[..], &s[..]));
        Ok(())
    }
}

impl GeneratedType {
    pub fn generate(&self) -> Result<(), String> {
        let tera = create_tera();
        let mut context = Context::new();
        context.add("fields", &self.fields);
        let s = unwrap_result(tera.render("data_type.tera", &context));
        let _ = unwrap_result(create_dir_all(&format!("{}/types", self.schema_dir)[..]));
        let _ = unwrap_result(file::write(&format!("{}/types/{}.json", self.schema_dir, self.name)[..], &s[..]));
        Ok(())
    }
}

impl GeneratedDefaultLog {
    pub fn generate(&self) -> Result<(), String> {
        let tera = create_tera();
        let mut context = Context::new();
        context.add("front_fields", &self.front_fields);
        context.add("back_fields", &self.back_fields);
        let s = unwrap_result(tera.render("default_log.tera", &context));
        let _ = unwrap_result(create_dir_all(&format!("{}", self.schema_dir)[..]));
        let _ = unwrap_result(file::write(&format!("{}/default.json", self.schema_dir)[..], &s[..]));
        Ok(())
    }
}

