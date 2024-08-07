#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    pub items: Box<super::Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unique_items: bool,
}
