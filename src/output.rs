extern crate tera;
use self::tera::Tera;

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
}

impl LogianOutput {

    fn output_compiled_file(
        &self,
        tera: &Tera,
        types: &UserDefinedTypes,
        log_schemas: &LogSchemas,
        default_schema: &DefaultLogSchema
    ) {
        let logs = ::template::generator::render_logs(tera, LOGS_TEMPLATE, log_schemas, default_schema, types);
        let data_types = ::template::generator::render_types(tera, TYPES_TEMPLATE, types);

        let _ = ::file::writer::write(&format!("{}/{}{}", self.output_dir, self.logs_file_name, self.file_suffix)[..], &logs[..]);
        let _ = ::file::writer::write(&format!("{}/{}{}", self.output_dir, self.types_file_name, self.file_suffix)[..], &data_types[..]);
    }

    fn output_files(
        &self,
        tera: &Tera,
        types: &UserDefinedTypes,
        log_schemas: &LogSchemas,
        default_schema: &DefaultLogSchema,
    ) {
        for schema in log_schemas.iter() {
            let log = ::template::generator::render_log(tera, LOG_TEMPLATE, &schema, default_schema, types);
            let _ = ::file::writer::write(&format!("{}/{}{}", self.output_dir, schema.name, self.file_suffix)[..], &log[..]);
        }

        for user_defined_type in types.iter() {
            let data_type = ::template::generator::render_type(tera, TYPE_TEMPLATE, &user_defined_type);
            let _ = ::file::writer::write(&format!("{}/{}{}", self.output_dir, user_defined_type.name(), self.file_suffix)[..], &data_type[..]);
        }
    }

    pub fn output(&self) {

        let types = ::parser::parse_user_defined_types(&format!("{}/types", self.schema_dir)[..]);
        let log_schemas = ::parser::parse_log_schemas(&format!("{}/schemas/logs", self.schema_dir)[..], &types);
        let default_schema = ::parser::parse_default_log_schema(&format!("{}/schemas/default.json", self.schema_dir)[..], &types);
        let mut tera = ::template::generator::find_templates(&self.plugin_dir[..], &self.plugin_name[..]);
        ::template::filter::register(&mut tera);

        if self.compiled {
            self.output_compiled_file(&tera, &types, &log_schemas, &default_schema);
        } else {
            self.output_files(&tera, &types, &log_schemas, &default_schema);
        }
    }
}
