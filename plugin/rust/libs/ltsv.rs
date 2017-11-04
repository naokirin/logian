#[allow(dead_code)]
pub fn escape(s: &String) -> String {
    s.replace("\\", "\\\\")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .to_string()
}
