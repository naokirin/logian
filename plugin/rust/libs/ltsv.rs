pub trait ToLtsv {
    fn to_ltsv(&self) -> String;
    fn to_ltsv_null() -> String;
}

impl<T> ToLtsv for Option<T> where T: ToLtsv {
    #[allow(dead_code)]
    fn to_ltsv(&self) -> String {
        match *self {
            None    => T::to_ltsv_null(),
            Some(ref x) => x.to_ltsv()
        }
    }

    #[allow(dead_code)]
    fn to_ltsv_null() -> String {
        "".to_string()
    }
}

impl ToLtsv for bool {
    #[allow(dead_code)]
    fn to_ltsv(&self) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_ltsv_null() -> String {
        "null".to_string()
    }
}

impl ToLtsv for i64 {
    #[allow(dead_code)]
    fn to_ltsv(&self) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_ltsv_null() -> String {
        "null".to_string()
    }
}

impl ToLtsv for f64 {
    #[allow(dead_code)]
    fn to_ltsv(&self) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_ltsv_null() -> String {
        "null".to_string()
    }
}

impl ToLtsv for String {
    #[allow(dead_code)]
    fn to_ltsv(&self) -> String {
        format!("{}", self)
    }

    #[allow(dead_code)]
    fn to_ltsv_null() -> String {
        "null".to_string()
    }
}


#[allow(dead_code)]
pub fn escape(s: &String) -> String {
    s.replace("\\", "\\\\")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .to_string()
}

#[allow(dead_code)]
pub fn escape_option(s: &Option<String>) -> String {
    if s.is_none() {
        return "null".to_string();
    }
    escape(&s.as_ref().unwrap())
}
