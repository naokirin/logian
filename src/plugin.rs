use std::path::Path;

pub struct Lib {
    pub path: String,
    pub to: String,
}

pub struct Setting {
    pub plugin_name: String,
    pub compiled: bool,
    pub file_suffix: String,
    pub libs: Vec<Lib>,
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
        .map(|value| Lib {
            path: value["path"].as_str().unwrap().to_string(),
            to: value["to"].as_str().unwrap().to_string(),
        }).collect();
    Setting {
        plugin_name: plugin_name.clone(),
        compiled: json["compiled"].as_bool().unwrap(),
        file_suffix: json["file_suffix"].as_str().unwrap().to_string(),
        libs: libs,
    }
}
