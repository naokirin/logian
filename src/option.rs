extern crate docopt;

docopt!(pub Args derive Debug, "
Usage:
  logian output <plugin> <schema-dir> <output-dir>
  logian generate (log|type) <log-name> [<field>...]
  logian (-h | --help)
  logian --version

Options:
  -h --help     Show this screen.
  --version     Show version.
");

pub fn parse() -> Args {
    Args::docopt().deserialize().unwrap_or_else(|e| e.exit())
}

pub struct Output {
    pub plugin_name: String,
    pub schema_dir: String,
    pub output_dir: String,
}

pub struct GeneratedField {
    pub field_name: String,
    pub data_type: String,
    pub nullable: bool,
}

pub enum GeneratedKind {
    Log,
    Type,
}

pub struct Generate {
    pub kind: GeneratedKind,
    pub name: String,
    pub fields: Vec<GeneratedField>,
}

impl Args {
    pub fn is_output(&self) -> bool {
        self.cmd_output
    }

    pub fn as_output(&self) -> Option<Output> {
        if !self.is_output() {
            return Option::None;
        }

        Some(Output {
            plugin_name: self.arg_plugin.clone(),
            schema_dir: self.arg_schema_dir.clone(),
            output_dir: self.arg_output_dir.clone(),
        })
    }

    pub fn is_generate(&self) -> bool {
        self.cmd_generate
    }

    pub fn as_generate(&self) -> Option<Generate> {
        if !self.is_generate() {
            return Option::None;
        }

        let fields = self.clone().arg_field.iter().map(|field| {
            let field_and_type: Vec<&str> = field.split(':').collect();

            if field_and_type.len() != 2 {
                panic!("Found invalid field format: {}.", field);
            }

            let mut data_type = field_and_type[1].to_string();
            let mut nullable = false;
            if data_type.chars().last().unwrap() == '?' {
                nullable = true;
                data_type.pop();
            }

            GeneratedField {
                field_name: field_and_type[0].to_string(),
                data_type: data_type,
                nullable: nullable,
            }

        }).collect();

        let mut kind = GeneratedKind::Log;
        if self.cmd_type {
            kind = GeneratedKind::Type;
        }

        Some(Generate {
            kind: kind,
            name: self.arg_log_name.clone(),
            fields: fields,
        })
    }
}

