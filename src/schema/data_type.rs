extern crate serde;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};

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

#[derive(Serialize)]
struct SerializedDataType {
    name: String,
    fields: Vec<Field>,
}

impl DataType {
    fn convert_serialized(&self) -> SerializedDataType {
        match *self {
            DataType::Boolean => SerializedDataType { name: "boolean".to_string(), fields: vec![] },
            DataType::Integer => SerializedDataType { name: "integer".to_string(), fields: vec![] },
            DataType::Float => SerializedDataType { name: "float".to_string(), fields: vec![] },
            DataType::String => SerializedDataType { name: "string".to_string(), fields: vec![] },
            DataType::DateTime => SerializedDataType { name: "datetime".to_string(), fields: vec![] },
            DataType::Timestamp => SerializedDataType { name: "timestamp".to_string(), fields: vec![] },
            DataType::Struct(ref name, ref fields) => SerializedDataType { name: name.clone(), fields: fields.clone() },
        }
    }

    pub fn name(&self) -> String {
        self.convert_serialized().name
    }
}

impl Serialize for DataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let data = self.convert_serialized();

        let mut state = serializer.serialize_struct("DataType", 2)?;
        state.serialize_field("name", &data.name)?;
        state.serialize_field("fields", &data.fields)?;
        state.end()
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

