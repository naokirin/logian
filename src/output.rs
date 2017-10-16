extern crate tera;

use self::tera::Tera;
use ::template::{ renderer, filter };
use ::plugin;
use ::file;
use ::parser;

type UserDefinedTypes = Vec<::schema::data_type::DataType>;
type LogSchemas = Vec<::schema::data_type::LogSchema>;
type DefaultLogSchema = ::schema::data_type::DefaultLogSchema;

const LOGS_TEMPLATE: &'static str = "logs.tera";
const LOG_TEMPLATE: &'static str = "log.tera";
const TYPES_TEMPLATE: &'static str = "types.tera";
const TYPE_TEMPLATE: &'static str = "type.tera";

pub struct LogianOutput {
    pub plugin_dir: String,
    pub plugin_name: String,
    pub schema_dir: String,
    pub output_dir: String,
    pub logs_file_name: String,
    pub types_file_name: String,
    pub file_suffix: String,
    pub compiled: bool,
    pub file_name_case: ::plugin::FileNameCase,
}

impl LogianOutput {

    fn output_compiled_file(
        &self,
        tera: &Tera,
        types: &UserDefinedTypes,
        log_schemas: &LogSchemas,
        default_schema: &DefaultLogSchema
    ) {
        let logs = renderer::render_logs(tera, LOGS_TEMPLATE, log_schemas, default_schema, types);
        let data_types = renderer::render_types(tera, TYPES_TEMPLATE, types);

        let logs_file_name = plugin::convert_case(&self.logs_file_name, &self.file_name_case);
        let types_file_name = plugin::convert_case(&self.types_file_name, &self.file_name_case);

        let _ = file::write(&format!("{}/{}{}", self.output_dir, logs_file_name, self.file_suffix)[..], &logs[..]);
        let _ = file::write(&format!("{}/{}{}", self.output_dir, types_file_name, self.file_suffix)[..], &data_types[..]);
    }

    fn output_files(
        &self,
        tera: &Tera,
        types: &UserDefinedTypes,
        log_schemas: &LogSchemas,
        default_schema: &DefaultLogSchema,
    ) {
        for schema in log_schemas.iter() {
            let log = renderer::render_log(tera, LOG_TEMPLATE, &schema, default_schema, types);
            let name = plugin::convert_case(&schema.name, &self.file_name_case);
            let _ = file::write(&format!("{}/{}{}", self.output_dir, name, self.file_suffix)[..], &log[..]);
        }

        for user_defined_type in types.iter() {
            let data_type = renderer::render_type(tera, TYPE_TEMPLATE, &user_defined_type);
            let name = plugin::convert_case(&user_defined_type.name(), &self.file_name_case);
            let _ = file::write(&format!("{}/{}{}", self.output_dir, name, self.file_suffix)[..], &data_type[..]);
        }
    }

    pub fn output(&self) -> Result<(), String> {

        let types = parser::parse_user_defined_types(&format!("{}/types", self.schema_dir)[..])?;
        let log_schemas = parser::parse_log_schemas(&format!("{}/logs", self.schema_dir)[..], &types)?;
        let default_schema = parser::parse_default_log_schema(&format!("{}/default.json", self.schema_dir)[..], &types)?;
        let mut tera = renderer::find_templates(&format!("{}/{}/templates", self.plugin_dir, self.plugin_name)[..]);
        filter::register(&mut tera);
        tera.autoescape_on(vec![]);

        if self.compiled {
            self.output_compiled_file(&tera, &types, &log_schemas, &default_schema);
        } else {
            self.output_files(&tera, &types, &log_schemas, &default_schema);
        }
        Ok(())
    }
}
