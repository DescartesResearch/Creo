#[derive(Debug, serde::Deserialize, serde::Serialize, serde_valid::Validate)]
pub struct User {
    #[serde(alias="_id")]
    pub id: String,
    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub username: String,

    #[validate(min_length=3)]
    #[validate(max_length=64)]
    pub email: String,

    #[validate(min_items=32)]
    #[validate(max_items=128)]
    pub password_hash: Vec<u8>,

    #[serde(default="chrono::Utc::now")]
    pub created_at: chrono::DateTime<chrono::Utc>
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionData {
    pub user_id: String,

    pub session_id: String,

    exp: chrono::DateTime<chrono::Utc>
}

impl SessionData {
    pub fn new(user_id: String, session_id: String) -> Self {
        Self {
            user_id,
            session_id,
            exp: get_expiry()
        }
    }

    pub fn exp(&self) -> chrono::DateTime<chrono::Utc> {
        self.exp
    }

}

fn get_expiry() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now() + chrono::TimeDelta::days(3)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionResponse {
    pub token: String,
    pub exp: chrono::DateTime<chrono::Utc>,
}

impl SessionResponse {
    pub fn new(token: String, exp: chrono::DateTime<chrono::Utc>) -> Self {
        Self { token, exp }
    }
}
