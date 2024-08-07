use serde::Deserialize;

#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct User {
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub username: String,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub email: String,

    #[validate(min_items=32)]
    #[validate(max_items=128)]
    #[serde(deserialize_with="hash_password", alias="password")]
    pub password_hash: Vec<u8>,

    #[serde(default="chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>
}

fn hash_password<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: serde::Deserializer<'de> {
    let password = String::deserialize(deserializer)?;

    Ok(crate::hash::hash_password(password))
}
