use serde::Deserialize;

#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct User {
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub username: Option<String>,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub email: Option<String>,

    #[validate(min_items=32)]
    #[validate(max_items=128)]
    #[serde(deserialize_with="hash_password")]
    #[serde(skip_serializing_if="Option::is_none", default)]
    pub password_hash: Option<Vec<u8>>,
}

fn hash_password<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where D: serde::Deserializer<'de> {
    let password: Option<String> = Option::deserialize(deserializer)?;

    if let Some(password) = password {
        return Ok(Some(crate::hash::hash_password(password)));
    }
    Ok(None)
}
