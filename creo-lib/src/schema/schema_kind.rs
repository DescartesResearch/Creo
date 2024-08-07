#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum SchemaKind {
    #[serde(untagged)]
    Type(Type),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    String(openapiv3::StringType),
    Number(openapiv3::NumberType),
    Integer(openapiv3::IntegerType),
    Object(super::object_type::ObjectType),
    Array(super::array_type::ArrayType),
    Boolean(openapiv3::BooleanType),
}
