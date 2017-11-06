extern crate docopt;
use std::env;

docopt!(pub Args derive Debug, "
Usage:
  logian init [--log-label=<ll>] [--schema-dir=<sd>]
  logian output <plugin> <output-dir> [--plugin-dir=<pd>] [--schema-dir=<sd>]
  logian generate (log|type) <name> [<field>...] [--comment=<cm>] [--schema-dir=<sd>]
  logian generate default-log [--front FRONT] [--back BACK] [--schema-dir=<sd>]
  logian (-h | --help)
  logian --version

Options:
  -h --help             Show this screen.
  --version             Show version.
  --log-label=<ll>      Set log name label [default: log].
  --plugin-dir=<pd>     Plugin directory [default: ].
  --schema-dir=<pd>     Schema directory [default: .].
  --comment=<cm>        Log schema comment [default: ].
  --front FRONT         Default log schema front fields [default: ].
  --back BACK           Default log schema back fields [default: ].
");

pub fn parse() -> Args {
    Args::docopt().deserialize().unwrap_or_else(|e| e.exit())
}

#[derive(Debug)]
pub struct Init {
    pub schema_dir: String,
    pub log_label: String,
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
    pub comment: String,
    pub schema_dir: String,
}

#[derive(Debug)]
pub struct GeneratedDefaultLog {
    pub front_fields: Vec<GeneratedField>,
    pub back_fields: Vec<GeneratedField>,
    pub schema_dir: String,
}

impl Args {

    pub fn is_init(&self) -> bool {
        self.cmd_init
    }

    pub fn as_init(&self) -> Result<Init, String> {
        if !self.is_init() {
            return Err("This argument is not init.".to_string());
        }

        Ok(Init {
            schema_dir: self.flag_schema_dir.to_string(),
            log_label: self.flag_log_label.to_string(),
        })
    }

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
            schema_dir: self.flag_schema_dir.to_string(),
            output_dir: self.arg_output_dir.clone(),
            plugin_dir: plugin_dir,
        })
    }

    pub fn is_type_generate(&self) -> bool {
        self.cmd_generate && self.cmd_type
    }

    pub fn is_log_generate(&self) -> bool {
        self.cmd_generate && self.cmd_log
    }

    pub fn is_default_log_generate(&self) -> bool {
        self.cmd_generate && self.cmd_default_log
    }

    fn get_fields(&self, fields: &Vec<String>) -> Result<Vec<GeneratedField>, String> {
        fields.iter().map(|field| {
            let field_and_type: Vec<&str> = field.split(':').map(|f| f.trim()).collect();

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

        }).collect::<Result<Vec<GeneratedField>, String>>()
    }

    fn as_generate(&self) -> Result<Generate, String> {
        let fields = self.get_fields(&self.clone().arg_field);
        if fields.is_err() {
            return Err(fields.unwrap_err());
        }

        let mut kind = GeneratedKind::Log;
        if self.cmd_type {
            kind = GeneratedKind::Type;
        }

        Ok(Generate {
            kind: kind,
            name: self.arg_name.clone(),
            fields: fields.unwrap(),
            comment: self.flag_comment.to_string(),
            schema_dir: self.flag_schema_dir.to_string(),
        })
    }

    pub fn as_log_generate(&self) -> Result<Generate, String> {
        if !self.is_log_generate() {
            return Err("This argument is not log generate.".to_string());
        }
        self.as_generate()
    }

    pub fn as_type_generate(&self) -> Result<Generate, String> {
        if !self.is_type_generate() {
            return Err("This argument is not type generate.".to_string());
        }
        self.as_generate()
    }

    fn parse_default_log_fields(&self, fields: &String) -> Vec<String> {
        fields.split(',').map(|f| f.trim().to_string()).collect()
    }

    pub fn as_default_log_generate(&self) -> Result<GeneratedDefaultLog, String> {
        if !self.is_default_log_generate() {
            return Err("This argument is not default log generate.".to_string());
        }
        let front_fields = self.get_fields(&self.parse_default_log_fields(&self.flag_front));
        let back_fields = self.get_fields(&self.parse_default_log_fields(&self.flag_back));
        if front_fields.is_err() {
            return Err(front_fields.unwrap_err());
        }
        if back_fields.is_err() {
            return Err(back_fields.unwrap_err());
        }

        Ok(GeneratedDefaultLog {
            front_fields: front_fields.unwrap(),
            back_fields: back_fields.unwrap(),
            schema_dir: self.flag_schema_dir.to_string(),
        })
    }
}

