extern crate tera;

use std::fmt::Debug;
use self::tera::{Tera, Context};
use ::template::renderer::find_templates;
use ::template::filter;

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct GeneratedField {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

pub struct GeneratedLog {
    pub template_dir: String,
    pub fields: Vec<GeneratedField>,
    pub comment: String,
}

pub struct GeneratedDefaultLog {
    pub template_dir: String,
    pub front_fields: Vec<GeneratedField>,
    pub back_fields: Vec<GeneratedField>,
}

pub struct GeneratedType {
    pub template_dir: String,
    pub fields: Vec<GeneratedField>,
    pub comment: String,
}

fn unwrap_result<T: Debug, E: Debug>(result: Result<T, E>) -> T {
    if result.is_ok() {
        return result.unwrap();
    }
    panic!("{:?}", result.unwrap_err());
}

fn create_tera(template_dir: &String) -> Tera {
    let mut tera = find_templates(&template_dir[..]);
    filter::register(&mut tera);
    return tera;
}

impl GeneratedLog {
    pub fn generate(&self) -> Result<String, String> {
        let tera = create_tera(&self.template_dir);
        let mut context = Context::new();
        context.add("fields", &self.fields);
        context.add("comment", &self.comment);
        let s = unwrap_result(tera.render("log_schema.tera", &context));
        Ok(s)
    }
}

impl GeneratedType {
    pub fn generate(&self) -> Result<String, String> {
        let tera = create_tera(&self.template_dir);
        let mut context = Context::new();
        context.add("fields", &self.fields);
        context.add("comment", &self.comment);
        let s = unwrap_result(tera.render("data_type.tera", &context));
        Ok(s)
    }
}

impl GeneratedDefaultLog {
    pub fn generate(&self) -> Result<String, String> {
        let tera = create_tera(&self.template_dir);
        let mut context = Context::new();
        context.add("front_fields", &self.front_fields);
        context.add("back_fields", &self.back_fields);
        let s = unwrap_result(tera.render("default_log.tera", &context));
        Ok(s)
    }
}

