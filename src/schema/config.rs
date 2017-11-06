use ::json;

pub struct Config {
    pub log_label: String,
}

pub fn parse(json: &str) -> Result<Config, String> {
    let parsed = json::parse(json)?;
    if !parsed.is_object() {
        return Err("schema.config root is not a object.".to_string());
    }

    let obj = parsed.as_object().unwrap();

    let log_label = obj.get("log_label");
    if log_label.is_none() {
        return Err("log_label is not found in schema.config.".to_string());
    }
    if !log_label.unwrap().is_string() {
        return Err("log_label is not string in schema.config.".to_string());
    }
    let log_label = log_label.unwrap().as_str().unwrap();
    Ok(Config {
        log_label: log_label.to_string(),
    })
}
