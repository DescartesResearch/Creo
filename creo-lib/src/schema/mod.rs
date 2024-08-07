mod array_type;
mod object_type;
mod schema_kind;

pub use array_type::ArrayType;
pub use object_type::ObjectType;
pub use schema_kind::{SchemaKind, Type};

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct Schema {
    #[serde(flatten)]
    pub schema_data: openapiv3::SchemaData,
    #[serde(flatten)]
    pub schema_kind: SchemaKind,
}

impl Schema {
    pub fn get_object_schema(&self) -> Option<&ObjectType> {
        match &self.schema_kind {
            SchemaKind::Type(type_schema) => match type_schema {
                Type::Object(object_type) => Some(object_type),
                _ => None,
            },
        }
    }

    pub fn get_array_schema_type(&self) -> Option<&ArrayType> {
        match &self.schema_kind {
            SchemaKind::Type(type_schema) => match type_schema {
                Type::Array(array_type) => Some(array_type),
                _ => None,
            },
        }
    }
}
