extern crate serde;

use self::serde::ser::{Serialize, Serializer};

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

impl DataType {
    pub fn name(&self) -> String {
        match *self {
            DataType::Boolean => "Boolean",
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::String => "String",
            DataType::DateTime => "DateTime",
            DataType::Timestamp => "Timestamp",
            DataType::Struct(ref name, _) => &name[..],
        }.to_string()
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let state = serializer.serialize_str(&self.name()[..])?;
        Result::Ok(state)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub enum TypeAttribute {
    None,
    Nullable
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Field {
    #[allow(dead_code)]
    pub name: String,

    #[allow(dead_code)]
    pub data_type: DataType,

    #[allow(dead_code)]
    pub type_attribute: TypeAttribute,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct LogSchema {
    #[allow(dead_code)]
    pub name: String,

    #[allow(dead_code)]
    pub fields: Vec<Field>,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct DefaultLogSchema {
    #[allow(dead_code)]
    pub front_fields: Vec<Field>,

    #[allow(dead_code)]
    pub back_fields: Vec<Field>,
}

