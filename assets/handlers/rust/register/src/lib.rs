mod db;
mod hash;
pub mod models;
mod read;
mod create;
mod unmarshal;


pub async fn register_user(json_data: impl AsRef<[u8]>) -> Option<models::User> {
    let user = unmarshal::unmarshal_user(json_data);
    if read::read_user_by_username(&user.username).await.is_some() {
        return None;
    }
    if read::read_user_by_email(&user.email).await.is_some() {
        return None;
    }
    let user = create::create_user(user).await;
    Some(user)
}

