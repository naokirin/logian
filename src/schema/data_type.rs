#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Boolean,
    Integer,
    Float,
    String,
    DateTime,
    Timestamp,
    Struct(String, Vec<Field>)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeAttribute {
    None,
    Nullable
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field {
    #[allow(dead_code)]
    pub name: String,

    #[allow(dead_code)]
    pub data_type: DataType,

    #[allow(dead_code)]
    pub type_attribute: TypeAttribute,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LogSchema {
    #[allow(dead_code)]
    pub name: String,

    #[allow(dead_code)]
    pub fields: Vec<Field>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DefaultLogSchema {
    #[allow(dead_code)]
    pub front_fields: Vec<Field>,

    #[allow(dead_code)]
    pub back_fields: Vec<Field>,
}

