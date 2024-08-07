#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTTPMethod {
    Get,
    Post,
}

impl serde::Serialize for HTTPMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HTTPMethod::Get => serializer.serialize_str("GET"),
            HTTPMethod::Post => serializer.serialize_str("POST"),
        }
    }
}
