use crate::models;
use serde_valid::json::FromJsonSlice;

pub fn unmarshal_invoice(json_data: impl AsRef<[u8]>) -> models::Invoice {
    models::Invoice::from_json_slice(json_data.as_ref()).unwrap()
}
