use crate::models;
use serde_valid::json::FromJsonSlice;

pub fn unmarshal_user(json_data: impl AsRef<[u8]>) -> models::User {
    models::User::from_json_slice(json_data.as_ref()).unwrap()
}
