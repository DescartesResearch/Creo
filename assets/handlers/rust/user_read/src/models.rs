#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct User {
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub username: String,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub email: String,

    #[serde(default="chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>
}
