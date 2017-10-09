extern crate fs_extra;
extern crate heck;

use std::path::Path;
use self::heck::{SnakeCase, MixedCase, CamelCase, ShoutySnakeCase};

pub struct Setting {
    pub plugin_name: String,
    pub compiled: bool,
    pub file_suffix: String,
    pub file_name_case: FileNameCase,
    pub libs: Vec<String>,
}

pub enum FileNameCase {
    UpperCamel,
    LowerCamel,
    Snake,
    ShoutySnake,
    None,
}

pub fn convert_case(s: &String, case: &FileNameCase) -> String {
    match *case {
        FileNameCase::UpperCamel  => s.to_camel_case(),
        FileNameCase::LowerCamel  => s.to_mixed_case(),
        FileNameCase::Snake       => s.to_snake_case(),
        FileNameCase::ShoutySnake => s.to_shouty_snake_case(),
        _                         => s.clone(),
    }
}

fn parse_file_name_case(s: &String) -> FileNameCase {
    match &s.to_camel_case()[..] {
        "UpperCamel" => FileNameCase::UpperCamel,
        "LowerCamel" => FileNameCase::LowerCamel,
        "Snake" => FileNameCase::Snake,
        "ShoutySnake" => FileNameCase::ShoutySnake,
        _ => FileNameCase::None,
    }
}

pub fn parse_setting(plugin_dir: &String, plugin_name: &String) -> Result<Setting, String> {
    let path = Path::new(&plugin_dir[..]).join(&plugin_name[..]).join("config.json");
    let path = path.to_str().unwrap();
    let json_file = ::file::read(&path[..]).unwrap();
    let json = ::json::parse(&json_file[..])?;
    let libs = &json["libs"];
    if !libs.is_array() {
        return Err(format!("{} is invalid libs item.", path))
    }
    let libs = libs.as_array().unwrap().into_iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();
    Ok(Setting {
        plugin_name: plugin_name.clone(),
        compiled: json["compiled"].as_bool().unwrap(),
        file_suffix: json["file_suffix"].as_str().unwrap().to_string(),
        file_name_case: parse_file_name_case(&json["file_name_case"].as_str().unwrap().to_string()),
        libs: libs,
    })
}

pub fn copy_libs(libs: &Vec<String>, plugin_path: &String, plugin_name: &String, output_dir: &String) {
    let path = Path::new(&plugin_path[..]).join(&plugin_name[..]);
    let libs = libs.into_iter()
        .map(|lib| path.join(lib.clone()))
        .collect();

    let options = fs_extra::dir::CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000,
    };
    fs_extra::copy_items(&libs, &output_dir[..], &options).unwrap();
}
