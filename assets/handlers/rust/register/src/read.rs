use crate::{db, models};


async fn read_user_by_key(key: &str, value: impl AsRef<str>) -> Option<models::User> {
    db::get_collection().await.find_one(mongodb::bson::doc! {key: value.as_ref()}).await.unwrap()
}

pub async fn read_user_by_username(username: impl AsRef<str>) -> Option<models::User> {
    read_user_by_key("username", username).await
}

pub async fn read_user_by_email(email: impl AsRef<str>) -> Option<models::User> {
    read_user_by_key("email", email).await
}
