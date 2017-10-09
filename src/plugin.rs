extern crate fs_extra;

use std::path::Path;

pub struct Setting {
    pub plugin_name: String,
    pub compiled: bool,
    pub file_suffix: String,
    pub libs: Vec<String>,
}

pub fn parse_setting(plugin_dir: &String, plugin_name: &String) -> Setting {
    let path = Path::new(&plugin_dir[..]).join(&plugin_name[..]).join("config.json");
    let path = path.to_str().unwrap();
    let json_file = ::file::reader::read(&path[..]).unwrap();
    let json = ::json::parser::parse_json(&json_file[..]);
    let libs = &json["libs"];
    if !libs.is_array() {
        panic!("{} is invalid libs item.", path);
    }
    let libs = libs.as_array().unwrap().into_iter()
        .map(|value| value.as_str().unwrap().to_string())
        .collect();
    Setting {
        plugin_name: plugin_name.clone(),
        compiled: json["compiled"].as_bool().unwrap(),
        file_suffix: json["file_suffix"].as_str().unwrap().to_string(),
        libs: libs,
    }
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
