extern crate docopt;
use std::env;

docopt!(pub Args derive Debug, "
Usage:
  logian output <plugin> <schema-dir> <output-dir> [--plugin-dir=<pd>]
  logian generate (log|type) <log-name> [<field>...]
  logian (-h | --help)
  logian --version

Options:
  -h --help         Show this screen.
  --version         Show version.
  --plugin-dir=<pd>  Plugin directory [default: ].
");

pub fn parse() -> Args {
    Args::docopt().deserialize().unwrap_or_else(|e| e.exit())
}

#[derive(Debug)]
pub struct Output {
    pub plugin_name: String,
    pub schema_dir: String,
    pub output_dir: String,
    pub plugin_dir: String,
}

#[derive(Debug)]
pub struct GeneratedField {
    pub field_name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[derive(Debug)]
pub enum GeneratedKind {
    Log,
    Type,
}

#[derive(Debug)]
pub struct Generate {
    pub kind: GeneratedKind,
    pub name: String,
    pub fields: Vec<GeneratedField>,
}

impl Args {
    pub fn is_output(&self) -> bool {
        self.cmd_output
    }

    pub fn as_output(&self) -> Result<Output, String> {
        if !self.is_output() {
            return Err("This argument is not output.".to_string());
        }

        let mut plugin_dir = self.flag_plugin_dir.to_string();
        if plugin_dir.is_empty() {
            let exe_path = env::current_exe().unwrap();
            let exe_dir = exe_path.parent().unwrap();
            plugin_dir = exe_dir.clone().join("plugin").to_str().unwrap().to_string();
        }

        Ok(Output {
            plugin_name: self.arg_plugin.clone(),
            schema_dir: self.arg_schema_dir.clone(),
            output_dir: self.arg_output_dir.clone(),
            plugin_dir: plugin_dir,
        })
    }

    pub fn is_generate(&self) -> bool {
        self.cmd_generate
    }

    pub fn as_generate(&self) -> Result<Generate, String> {
        if !self.is_generate() {
            return Err("This argument is not generate.".to_string());
        }

        let fields = self.clone().arg_field.iter().map(|field| {
            let field_and_type: Vec<&str> = field.split(':').collect();

            if field_and_type.len() != 2 {
                return Err(format!("Found invalid field format: {}.", field));
            }

            let mut data_type = field_and_type[1].to_string();
            let mut nullable = false;
            if data_type.chars().last().unwrap() == '?' {
                nullable = true;
                data_type.pop();
            }

            Ok(GeneratedField {
                field_name: field_and_type[0].to_string(),
                data_type: data_type,
                nullable: nullable,
            })

        }).collect::<Result<Vec<GeneratedField>, String>>();

        if fields.is_err() {
            return Err(fields.unwrap_err());
        }

        let mut kind = GeneratedKind::Log;
        if self.cmd_type {
            kind = GeneratedKind::Type;
        }

        Ok(Generate {
            kind: kind,
            name: self.arg_log_name.clone(),
            fields: fields.unwrap(),
        })
    }
}

