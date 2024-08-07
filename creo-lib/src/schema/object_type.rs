use indexmap::IndexMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, Box<super::Schema>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Any(bool),
    Schema(Box<super::Schema>),
}

impl ObjectType {
    pub fn is_free_form_object(&self) -> bool {
        match self.additional_properties {
            Some(AdditionalProperties::Any(true)) | None => self.properties.is_empty(),
            _ => false,
        }
    }
}
