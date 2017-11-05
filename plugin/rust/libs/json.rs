use std::ops::Fn;

pub enum JsonEscapingType {
    Default,
    WithoutMultibytes,
}

pub trait ToJson {
    fn to_json(&self, escaping_type: &JsonEscapingType) -> String;
    fn to_json_null() -> String;
}

impl<T> ToJson for Option<T> where T: ToJson {
    #[allow(dead_code)]
    fn to_json(&self, escaping_type: &JsonEscapingType) -> String {
        match *self {
            None    => T::to_json_null(),
            Some(ref x) => x.to_json(escaping_type)
        }
    }

    #[allow(dead_code)]
    fn to_json_null() -> String {
        "".to_string()
    }
}

impl ToJson for bool {
    #[allow(dead_code)]
    fn to_json(&self, _escaping_type: &JsonEscapingType) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_json_null() -> String {
        "null".to_string()
    }
}

impl ToJson for i64 {
    #[allow(dead_code)]
    fn to_json(&self, _escaping_type: &JsonEscapingType) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_json_null() -> String {
        "null".to_string()
    }
}

impl ToJson for f64 {
    #[allow(dead_code)]
    fn to_json(&self, _escaping_type: &JsonEscapingType) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_json_null() -> String {
        "null".to_string()
    }
}

impl ToJson for String {
    #[allow(dead_code)]
    fn to_json(&self, escaping_type: &JsonEscapingType) -> String {
        match *escaping_type {
            JsonEscapingType::Default           => escape(self),
            JsonEscapingType::WithoutMultibytes => escape_without_multibytes(self),
        }
    }

    #[allow(dead_code)]
    fn to_json_null() -> String {
        "null".to_string()
    }
}


#[allow(dead_code)]
fn escape_multibyte_char(c :&u16) -> String {
    format!("\\u{:>04x}", c)
}

#[allow(dead_code)]
fn escape_with(s: &String, f: &Fn(&u16) -> String) -> String {
    let input: ::std::str::EncodeUtf16 = s.encode_utf16();
    let mut output: Vec<u8> = Vec::with_capacity(input.clone().count());
    for c in input {
        match c {
            0x22          => output.extend_from_slice(b"\\\""),
            0x2F          => output.extend_from_slice(b"\\/"),
            0x5C          => output.extend_from_slice(b"\\\\"),
            0x0A          => output.extend_from_slice(b"\\n"),
            0x0D          => output.extend_from_slice(b"\\r"),
            0x09          => output.extend_from_slice(b"\\t"),
            0x08          => output.extend_from_slice(b"\\b"),
            0x0C          => output.extend_from_slice(b"\\f"),
            _ if c > 0x7F => output.extend(f(&c).into_bytes()),
            _             => output.extend(String::from_utf16(&[c]).unwrap().into_bytes()),
        }
    }
    String::from_utf8(output).unwrap()
}

#[allow(dead_code)]
pub fn escape_without_multibytes(s: &String) -> String {
    escape_with(s, &|c| String::from_utf16(&[*c]).unwrap())
}

#[allow(dead_code)]
pub fn escape(s: &String) -> String {
    escape_with(s, &|c| escape_multibyte_char(c))
}

