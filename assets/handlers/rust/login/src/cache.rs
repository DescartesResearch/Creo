use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::{RngCore, SeedableRng};
use base64::prelude::*;

use crate::models;

lazy_static!{
    pub static ref CACHE: std::sync::RwLock<SessionRepository> = std::sync::RwLock::new(SessionRepository::new());
}


pub struct SessionRepository {
    cache: HashMap<String, models::SessionData>
}

impl SessionRepository {
    pub fn new() -> Self {
        Self { cache: HashMap::default() }
    }
    pub fn set_new_session(&mut self, user_id: String) -> models::SessionResponse {
        let mut bytes = [0; 24];
        rand_chacha::ChaCha20Rng::from_entropy().fill_bytes(&mut bytes);
        let session_id = BASE64_URL_SAFE.encode(bytes);
        let session_data = models::SessionData::new(user_id.clone(), session_id.clone());
        let session = models::SessionResponse::new(session_id.clone(), session_data.exp());
        self.cache.insert(session_id, session_data);
        session
    }
}
