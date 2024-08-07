mod cache;
pub mod db;
mod hash;
pub mod models;
mod read;

use argon2::PasswordVerifier;


use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: regex::Regex = regex::Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
}

pub async fn login_with_username_or_email(username_or_email: String, password: String) -> Option<models::SessionResponse>{
    let user = if EMAIL_REGEX.is_match(&username_or_email) {
        read::read_user_by_email(&username_or_email).await
    } else {
        read::read_user_by_username(&username_or_email).await
    };

    if let Some(user) = user {
        let password_hash = std::str::from_utf8(&user.password_hash).unwrap();
        let password_hash = argon2::PasswordHash::new(password_hash).unwrap();
        if hash::HASHER.verify_password(password.as_bytes(), &password_hash).is_ok() {
            let mut repo = cache::CACHE.write().unwrap();
            return Some(repo.set_new_session(user.id.clone()));
        };
    }

    None
}


